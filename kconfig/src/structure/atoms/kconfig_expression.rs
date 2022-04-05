use crate::structure::atoms::kconfig_symbol::KconfigSymbol;

#[derive(Clone, Debug, PartialEq)]
pub struct KconfigExpression {
    pub source: String,
    // pub included_symbols: Vec<KconfigSymbol>,
}

impl KconfigExpression {
    pub fn new(source: String) -> Self {
        KconfigExpression { source }
    }

    pub fn included_symbols(&self) -> Vec<KconfigSymbol> {
        const OPERATORS: &str = "&=()|!";
        let mut rip_string = self.source.clone();
        for op in OPERATORS.chars() {
            rip_string = rip_string.replace(op, " ");
        }
        rip_string
            .split_whitespace()
            .filter(|el| !el.starts_with('$'))
            .map(|s| KconfigSymbol {
                name: s.to_string(),
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::structure::atoms::kconfig_expression::KconfigExpression;
    use crate::structure::atoms::kconfig_symbol::KconfigSymbol;

    #[test]
    fn happy_path_symbol_extraction_works() {
        let soruce = "!ASDF && KEKWUT || (LOL||K)".to_string();
        let expression = KconfigExpression::new(soruce);
        assert_eq!(
            vec![
                KconfigSymbol::new("ASDF".to_string()),
                KconfigSymbol::new("KEKWUT".to_string()),
                KconfigSymbol::new("LOL".to_string()),
                KconfigSymbol::new("K".to_string()),
            ],
            expression.included_symbols()
        )
    }
}
