use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Path, Result as SynResult};

use super::{ArgListType, MinimalAttrArgs};

pub trait ArgHandler {
    fn conv_func_name(&self) -> &str;

    fn create_bulk_impl(
        &self,
        name: &Ident,
        stmts: TokenStream,
        first: Path,
        last: Path,
        second_last: Option<Path>,
    ) -> TokenStream;

    fn create_pair_impl(&self, name: &Ident, first: &Path, last: &Path) -> TokenStream;

    fn stmt_end(&self) -> TokenStream {
        TokenStream::new()
    }

    fn make_impl(&self, name: &Ident, args: ArgListType) -> SynResult<TokenStream> {
        match args {
            ArgListType::Simple(args) => self.make_bulk_impl(name, args),
            ArgListType::All(args) => self.make_pair_impls(name, args),
        }
    }

    /// Processes an argument list considering regular behavior
    /// of implementing the trait only between source type and target type.
    fn make_bulk_impl(
        &self,
        name: &Ident,
        args: MinimalAttrArgs,
    ) -> SynResult<TokenStream> {
        let MinimalAttrArgs {
            first,
            mut last,
            iter,
        } = args;

        let stmt_end = self.stmt_end();
        let func = TokenStream::from_str(self.conv_func_name())?;

        let mut second_last = None;

        // Create the buffer and store the minimum amount of statements.
        let mut stmts = TokenStream::new();
        stmts.extend(quote! {let interm = #first::#func(val)#stmt_end;});
        stmts.extend(quote! {let interm = #last::#func(interm)#stmt_end;});

        // Store other statements, if any
        for param in iter {
            second_last = Some(last);
            last = param?;
            stmts.extend(quote! {let interm = #last::#func(interm)#stmt_end;});
        }

        // Generate code
        let expanded = self.create_bulk_impl(name, stmts, first, last, second_last);
        Ok(expanded)
    }

    /// Processes an argument list considering the enhanced behavior
    /// of implementing the trait between all transitions from either
    /// one source and multiple targets or multiple targets and once source,
    /// depending on the trait.
    fn make_pair_impls(
        &self,
        name: &Ident,
        args: MinimalAttrArgs,
    ) -> SynResult<TokenStream> {
        let MinimalAttrArgs {
            mut first,
            mut last,
            iter,
        } = args;

        // Create the buffer and store the first impl.
        let mut impls = TokenStream::new();
        impls.extend(self.create_pair_impl(name, &first, &last));

        // Create and store other impls, if any
        for param in iter {
            first = last;
            last = param?;
            impls.extend(self.create_pair_impl(name, &first, &last));
        }

        Ok(impls)
    }
}
