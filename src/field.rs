use derive_more::{Constructor, Deref};
use syn::{parse_quote_spanned, spanned::Spanned, ExprField, Index, Member};

#[derive(Clone, Debug, Constructor, Deref)]
pub struct Field {
    #[deref]
    pub field: syn::Field,
    pub idx: usize,
}

impl Field {
    pub fn name(&self) -> ExprField {
        match self.field.ident {
            Some(ref name) => parse_quote_spanned! { self.field.span() =>
                self.#name
            },
            None => {
                let idx = Member::Unnamed(Index {
                    index: self.idx as u32,
                    span: self.field.span(),
                });

                parse_quote_spanned! { self.field.span() =>
                    self.#idx
                }
            }
        }
    }
}
