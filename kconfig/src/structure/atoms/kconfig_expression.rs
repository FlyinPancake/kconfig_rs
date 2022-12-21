use crate::structure::atoms::kconfig_symbol::KconfigSymbol;

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct KconfigExpression {
    pub source: String,
}

const OPERATORS: &str = "yn&=()|!\\;m'";

impl KconfigExpression {
    pub fn new(source: String) -> Self {
        KconfigExpression { source }
    }

    pub fn included_symbols(&self) -> Vec<KconfigSymbol> {
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

    pub fn check_if_met(&self, set_options: &Vec<KconfigSymbol>) -> bool {
        if self.source.contains("(") && self.source.contains(")") {}
        false
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

    fn happy_path_not_symbol_works() {
        let source = KconfigExpression::new("!ASDF".to_string());
        let set_symbols: Vec<KconfigSymbol> = vec![];
        assert!(source.check_if_met(&set_symbols))
    }
}
