// Copyright (C) 2024 Bellande Architecture Mechanism Research Innovation Center, Ronaldson Bellande

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{parse_file, Item};

pub struct Module {
    pub name: String,
    pub path: PathBuf,
    pub ast: syn::File,
    pub symbols: HashMap<String, TokenStream>,
}

pub struct Importer {
    modules: HashMap<String, Module>,
    search_paths: Vec<PathBuf>,
}

impl Importer {
    pub fn new() -> Self {
        Importer {
            modules: HashMap::new(),
            search_paths: vec![PathBuf::from(".")],
        }
    }

    pub fn add_search_path(&mut self, path: PathBuf) {
        self.search_paths.push(path);
    }

    fn find_module(&self, module_name: &str) -> Option<PathBuf> {
        for search_path in &self.search_paths {
            let module_path = search_path.join(format!("{}.rs", module_name));
            if module_path.exists() {
                return Some(module_path);
            }
        }
        None
    }

    pub fn import(&mut self, module_name: &str) -> Result<&Module, String> {
        if !self.modules.contains_key(module_name) {
            let module_path = self
                .find_module(module_name)
                .ok_or_else(|| format!("Module '{}' not found", module_name))?;

            let contents = fs::read_to_string(&module_path)
                .map_err(|e| format!("Failed to read module '{}': {}", module_name, e))?;

            let ast = parse_file(&contents)
                .map_err(|e| format!("Failed to parse module '{}': {}", module_name, e))?;

            let mut symbols = HashMap::new();
            for item in &ast.items {
                match item {
                    Item::Fn(f) => {
                        symbols.insert(f.sig.ident.to_string(), f.to_token_stream());
                    }
                    Item::Struct(s) => {
                        symbols.insert(s.ident.to_string(), s.to_token_stream());
                    }
                    Item::Enum(e) => {
                        symbols.insert(e.ident.to_string(), e.to_token_stream());
                    }
                    Item::Const(c) => {
                        symbols.insert(c.ident.to_string(), c.to_token_stream());
                    }
                    Item::Static(s) => {
                        symbols.insert(s.ident.to_string(), s.to_token_stream());
                    }
                    Item::Trait(t) => {
                        symbols.insert(t.ident.to_string(), t.to_token_stream());
                    }
                    Item::Impl(i) => {
                        if let Some((_, trait_path, _)) = &i.trait_ {
                            symbols.insert(
                                format!("impl_{}", trait_path.segments.last().unwrap().ident),
                                i.to_token_stream(),
                            );
                        } else if let Some(t) = &i.self_ty {
                            symbols
                                .insert(format!("impl_{}", quote::quote!(#t)), i.to_token_stream());
                        }
                    }
                    _ => {}
                }
            }

            self.modules.insert(
                module_name.to_string(),
                Module {
                    name: module_name.to_string(),
                    path: module_path,
                    ast,
                    symbols,
                },
            );
        }

        Ok(self.modules.get(module_name).unwrap())
    }

    pub fn get_module(&self, module_name: &str) -> Option<&Module> {
        self.modules.get(module_name)
    }

    pub fn get_symbol(&self, module_name: &str, symbol_name: &str) -> Option<&TokenStream> {
        self.modules.get(module_name)?.symbols.get(symbol_name)
    }
}

#[macro_export]
macro_rules! import {
    ($importer:expr, $module:expr) => {
        $importer.import($module).unwrap()
    };
}

#[macro_export]
macro_rules! from_import {
    ($importer:expr, $module:expr, $($item:ident),+) => {
        $(
            let $item = $importer.get_symbol($module, stringify!($item))
                .unwrap_or_else(|| panic!("Symbol '{}' not found in module '{}'", stringify!($item), $module));
        )+
    };
}
