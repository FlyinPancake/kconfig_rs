use crate::structure::atoms::KconfigSymbol;
use crate::structure::kconfig_node::KconfigNode;

#[derive(Debug, Clone)]
pub struct KconfigNodeChildren {
    pub children: Vec<KconfigNode>,
}

impl KconfigNodeChildren {
    pub(crate) fn new_empty() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub(crate) fn add_children(&mut self, node: KconfigNode) {
        self.children.push(node);
    }

    pub(crate) fn add_all_children(&mut self, children: KconfigNodeChildren) {
        for chd in children.children {
            self.children.push(chd);
        }
    }

    pub fn get_all_symbols(&self) -> Vec<KconfigSymbol> {
        self.children
            .iter()
            .flat_map(|node| match node {
                KconfigNode::Config(config) => {
                    vec![config.symbol.clone()]
                }
                KconfigNode::Menu(menu) => menu.children.get_all_symbols(),
                KconfigNode::MenuConfig(menu_config) => {
                    vec![menu_config.symbol.clone()]
                }
                KconfigNode::If(if_node) => if_node.children.get_all_symbols(),
            })
            .collect::<Vec<KconfigSymbol>>()
    }
}

#[cfg(test)]
mod test {
    use crate::parser::kconfig_parser_impl::parser_traits::{
        ParseableWithUnknownSpan, ParsingContext,
    };
    use crate::parser::utils::parse_span::ParseSpan;
    use crate::structure::atoms::{
        KconfigDependency, KconfigExpression, KconfigReverseDependency, KconfigSymbol,
    };
    use crate::structure::kconfig_config::KconfigConfig;
    use crate::structure::kconfig_node::KconfigNode::{Config, Menu};
    use crate::structure::kconfig_node_children::KconfigNodeChildren;
    use crate::structure::nodes::{KconfigConfigNode, KconfigMenuNode};
    use crate::structure::property::{
        ConfigType, KconfigDependenciesProperty, KconfigHelpProperty,
        KconfigReverseDependenciesProperty, KconfigTypeProperty,
    };

    #[test]
    fn happy_path_get_all_symbols() {
        let children = KconfigNodeChildren {
            children: vec![
                Config(KconfigConfigNode {
                    symbol: KconfigSymbol::new("MKAY_LOL".to_string()),
                    config: KconfigConfig {
                        type_property: Option::from(KconfigTypeProperty {
                            config_type: ConfigType::Bool,
                            if_dep_on_type: None,
                        }),
                        dependencies: KconfigDependenciesProperty {
                            dependencies: vec![KconfigDependency {
                                expression: KconfigExpression {
                                    source: "HOORAH && OH_YEAH".to_string(),
                                },
                            }],
                        },
                        reverse_dependencies: KconfigReverseDependenciesProperty {
                            dependencies: vec![KconfigReverseDependency {
                                on_symbol: KconfigSymbol::new("GOING_BACKWARDS".to_string()),
                                if_constraint: None,
                            }],
                        },
                        help_property: None,
                    },
                }),
                Config(KconfigConfigNode {
                    symbol: KconfigSymbol::new("HOORAH".to_string()),
                    config: KconfigConfig {
                        type_property: Option::from(KconfigTypeProperty {
                            config_type: ConfigType::Tristate,
                            if_dep_on_type: None,
                        }),
                        dependencies: KconfigDependenciesProperty {
                            dependencies: vec![],
                        },
                        reverse_dependencies: KconfigReverseDependenciesProperty {
                            dependencies: vec![],
                        },
                        help_property: None,
                    },
                }),
                Menu(KconfigMenuNode {
                    name: "Some menu".to_string(),
                    dependencies: KconfigDependenciesProperty {
                        dependencies: vec![],
                    },
                    children: KconfigNodeChildren {
                        children: vec![Config(KconfigConfigNode {
                            symbol: KconfigSymbol::new("BAR".to_string()),
                            config: KconfigConfig {
                                type_property: None,
                                dependencies: KconfigDependenciesProperty {
                                    dependencies: vec![],
                                },
                                reverse_dependencies: KconfigReverseDependenciesProperty {
                                    dependencies: vec![],
                                },
                                help_property: None,
                            },
                        })],
                    },
                }),
            ],
        };

        assert_eq!(
            children.get_all_symbols(),
            vec![
                KconfigSymbol::new("MKAY_LOL".to_string()),
                KconfigSymbol::new("HOORAH".to_string()),
                KconfigSymbol::new("BAR".to_string())
            ]
        );
        // assert_eq!(parse_res.0.help_text, "endmenu lol\nkeke sajt")
    }
}
