use derive_syn_parse::Parse;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
	parse_macro_input,
	spanned::Spanned,
	token::{Comma, Gt, Lt},
	Error, Expr, FnArg, ItemFn, ItemMod, LitInt, Pat, Result, Stmt, Type,
};

mod keywords {
	syn::custom_keyword!(extrinsic_call);
}

fn emit_error<T: Into<TokenStream> + Clone, S: Into<String>>(item: &T, message: S) -> TokenStream {
	let item = Into::<TokenStream>::into(item.clone());
	let message = Into::<String>::into(message);
	let span = proc_macro2::TokenStream::from(item).span();
	return syn::Error::new(span, message).to_compile_error().into()
}

#[derive(Debug, Clone, PartialEq)]
struct ParamDef {
	name: String,
	typ: Type,
	start: u32,
	end: u32,
}

#[derive(Parse)]
struct RangeArgs {
	_lt_token: Lt,
	start: LitInt,
	_comma: Comma,
	end: LitInt,
	_gt_token: Gt,
}

struct BenchmarkDef {
	params: Vec<ParamDef>,
	setup_stmts: Vec<Stmt>,
	extrinsic_call_stmt: Stmt,
	verify_stmts: Vec<Stmt>,
}

impl BenchmarkDef {
	pub fn from(item_fn: &ItemFn) -> Result<BenchmarkDef> {
		let mut i = 0; // index of child
		let mut params: Vec<ParamDef> = Vec::new();
		for arg in &item_fn.sig.inputs {
			// parse params such as "x: Linear<0, 1>"
			let mut name: Option<String> = None;
			let mut typ: Option<&Type> = None;
			let mut start: Option<u32> = None;
			let mut end: Option<u32> = None;
			if let FnArg::Typed(arg) = arg {
				if let Pat::Ident(ident) = &*arg.pat {
					name = Some(ident.ident.to_token_stream().to_string());
				}
				let tmp = &*arg.ty;
				typ = Some(tmp);
				if let Type::Path(tpath) = tmp {
					if let Some(segment) = tpath.path.segments.last() {
						let args = segment.arguments.to_token_stream().into();
						if let Ok(args) = syn::parse::<RangeArgs>(args) {
							if let Ok(start_parsed) = args.start.base10_parse::<u32>() {
								start = Some(start_parsed);
							}
							if let Ok(end_parsed) = args.end.base10_parse::<u32>() {
								end = Some(end_parsed);
							}
						}
					}
				}
			}
			if let (Some(name), Some(typ), Some(start), Some(end)) = (name, typ, start, end) {
				// if true, this iteration of param extraction was successful
				params.push(ParamDef { name, typ: typ.clone(), start, end });
			} else {
				return Err(Error::new(
					arg.span(),
					"Invalid benchmark function param. A valid example would be `x: Linear<5, 10>`.",
				))
			}
		}
		for child in &item_fn.block.stmts {
			// find #[extrinsic_call] annotation and build up the setup, call, and verify
			// blocks based on the location of this annotation
			if let Stmt::Semi(Expr::Call(fn_call), token) = child {
				let mut k = 0; // index of attr
				for attr in &fn_call.attrs {
					if let Some(segment) = attr.path.segments.last() {
						if let Ok(_) = syn::parse::<keywords::extrinsic_call>(
							segment.ident.to_token_stream().into(),
						) {
							let mut fn_call_copy = fn_call.clone();
							fn_call_copy.attrs.remove(k); // consume #[extrinsic call]
							return Ok(BenchmarkDef {
								params,
								setup_stmts: Vec::from(&item_fn.block.stmts[0..i]),
								extrinsic_call_stmt: Stmt::Semi(
									Expr::Call(fn_call_copy),
									token.clone(),
								),
								verify_stmts: Vec::from(
									&item_fn.block.stmts[(i + 1)..item_fn.block.stmts.len()],
								),
							})
						}
					}
					k += 1;
				}
			}
			i += 1;
		}
		return Err(Error::new(
			item_fn.block.brace_token.span,
			"Missing #[extrinsic_call] annotation in benchmark function body.",
		))
	}
}

pub fn benchmarks(_attrs: TokenStream, tokens: TokenStream) -> TokenStream {
	let item_mod = parse_macro_input!(tokens as ItemMod);
	let contents = match item_mod.content {
		Some(content) => content.1,
		None =>
			return emit_error(
				&item_mod.to_token_stream(),
				"#[frame_support::benchmarks] can only be applied to a non-empty module.",
			),
	};
	let mod_ident = item_mod.ident;
	quote! {
		#[cfg(any(feature = "runtime-benchmarks", test))]
		mod #mod_ident {
			#(#contents)
			*
		}
	}
	.into()
}

pub fn benchmark(_attrs: TokenStream, tokens: TokenStream) -> TokenStream {
	let item_fn = parse_macro_input!(tokens as ItemFn);
	let benchmark_def = match BenchmarkDef::from(&item_fn) {
		Ok(def) => def,
		Err(err) => return err.to_compile_error().into(),
	};
	let name = item_fn.sig.ident;
	let krate = quote!(::frame_benchmarking);
	let support = quote!(::frame_support);
	let setup_stmts = benchmark_def.setup_stmts;
	let extrinsic_call_stmt = benchmark_def.extrinsic_call_stmt;
	let verify_stmts = benchmark_def.verify_stmts;
	let params = vec![quote!(x, 0, 1)];
	let param_names = vec![quote!(x)];
	quote! {
		#support::assert_impl_all!(#support::Linear<0, 1>: #support::ParamRange);

		#[allow(non_camel_case_types)]
		struct #name;

		#[allow(unused_variables)]
		impl<T: Config> ::frame_benchmarking::BenchmarkingSetup<T>
		for #name {
			fn components(&self) -> #krate::Vec<(#krate::BenchmarkParameter, u32, u32)> {
				#krate::vec! [
					#(
						(#krate::BenchmarkParameter::#params)
					),*
				]
			}

			fn instance(
				&self,
				components: &[(#krate::BenchmarkParameter, u32)],
				verify: bool
			) -> Result<#krate::Box<dyn FnOnce() -> Result<(), #krate::BenchmarkError>>, #krate::BenchmarkError> {
				#(
					// prepare instance #param_names
					let #param_names = components.iter()
						.find(|&c| c.0 == #krate::BenchmarkParameter::#param_names)
						.ok_or("Could not find component during benchmark preparation.")?
						.1;
				)*

				// TODO: figure out parameter parsing:
				// $(
				// 	let $pre_id : $pre_ty = $pre_ex;
				// )*
				// $( $param_instancer ; )*
				// $( $post )*

				// benchmark setup code (stuff before #[extrinsic_call])
				#(
					#setup_stmts
				)*

				Ok(#krate::Box::new(move || -> Result<(), #krate::BenchmarkError> {
					#extrinsic_call_stmt
					if verify {
						#(
							#verify_stmts
						)*
					}
					Ok(())
				}))
			}
		}
	}
	.into()
}
