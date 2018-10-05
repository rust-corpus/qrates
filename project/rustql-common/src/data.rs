//
// This file contains several data structures to represent data extracted from
// `rustc::hir`. They can be serialized into several crate packages and then be
// 'linked' together.
//

use crate::rustc::hir::map::definitions::{DefPath, DefPathData};
use crate::rustc::hir;
use crate::rustc::hir::def_id::DefIndex;
use serde::{Serializer, Deserializer, Serialize, Deserialize};


/// Structure that identifies a crate uniquely.
/// Two crates with the same CrateIdentifier are guaranteed to have the same ast.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CrateIdentifier {
	pub name: String,
	pub version: (u64, u64, u64),
	pub config_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crate {
	pub metadata: CrateIdentifier,
	pub mods: Vec<Mod>,
	pub functions: Vec<Function>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mod {
	pub name: String,
	pub parent_mod: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Function {
	pub name: String,
	pub is_unsafe: bool,
	pub calls: Vec<GlobalDefPath>,
	pub containing_mod: Option<usize>,
	//#[serde(skip_serializing, skip_deserializing)]
	//pub def_id: DefIdWrapper
}

/// Analogous to rustc::hir::map::definitions::DefPathData
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GlobalDefPathData {
	CrateRoot,
	Misc,
	Impl,
	Trait(String),
	AssocTypeInTrait(String),
	AssocTypeInImpl(String),
	AssocExistentialInImpl(String),
	TypeNs(String),
	ValueNs(String),
	Module(String),
	MacroDef(String),
	ClosureExpr,
	TypeParam(String),
	LifetimeParam(String),
	EnumVariant(String),
	Field(String),
	StructCtor,
	AnonConst,
	ImplTrait,
	GlobalMetaData(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalDisambiguatedDefPathData {
	pub data: GlobalDefPathData,
	pub disambiguator: u32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalDefPath {
	pub path: Vec<GlobalDisambiguatedDefPathData>,
	pub crate_ident: CrateIdentifier
}

impl From<&hir::map::definitions::DisambiguatedDefPathData> for GlobalDisambiguatedDefPathData {
	fn from(dpd: &hir::map::definitions::DisambiguatedDefPathData) -> Self {
		use self::hir::map::definitions::DefPathData;
		GlobalDisambiguatedDefPathData {
			data: match dpd.data {
				DefPathData::CrateRoot => GlobalDefPathData::CrateRoot,
				DefPathData::Misc => GlobalDefPathData::Misc,
				DefPathData::Impl => GlobalDefPathData::Impl,
				DefPathData::Trait(id) => GlobalDefPathData::Trait(id.with(str::to_owned)),
				DefPathData::AssocTypeInTrait(id) => GlobalDefPathData::AssocTypeInTrait(id.with(str::to_owned)),
				DefPathData::AssocTypeInImpl(id) => GlobalDefPathData::AssocTypeInImpl(id.with(str::to_owned)),
				DefPathData::AssocExistentialInImpl(id) => GlobalDefPathData::AssocExistentialInImpl(id.with(str::to_owned)),
				DefPathData::TypeNs(id) => GlobalDefPathData::TypeNs(id.with(str::to_owned)),
				DefPathData::ValueNs(id) => GlobalDefPathData::ValueNs(id.with(str::to_owned)),
				DefPathData::Module(id) => GlobalDefPathData::Module(id.with(str::to_owned)),
				DefPathData::MacroDef(id) => GlobalDefPathData::MacroDef(id.with(str::to_owned)),
				DefPathData::ClosureExpr => GlobalDefPathData::ClosureExpr,
				DefPathData::TypeParam(id) => GlobalDefPathData::TypeParam(id.with(str::to_owned)),
				DefPathData::LifetimeParam(id) => GlobalDefPathData::LifetimeParam(id.with(str::to_owned)),
				DefPathData::EnumVariant(id) => GlobalDefPathData::EnumVariant(id.with(str::to_owned)),
				DefPathData::Field(id) => GlobalDefPathData::Field(id.with(str::to_owned)),
				DefPathData::StructCtor => GlobalDefPathData::StructCtor,
				DefPathData::AnonConst => GlobalDefPathData::AnonConst,
				DefPathData::ImplTrait => GlobalDefPathData::ImplTrait,
				DefPathData::GlobalMetaData(id) => GlobalDefPathData::GlobalMetaData(id.with(str::to_owned)),
			},
			disambiguator: dpd.disambiguator
		}
	}
}

impl GlobalDefPath {
	pub fn new(def: &hir::map::definitions::DefPath, c: &CrateIdentifier) -> Self {
		GlobalDefPath {
			path: def.data.iter().map(GlobalDisambiguatedDefPathData::from).collect::<Vec<GlobalDisambiguatedDefPathData>>(),
			crate_ident: c.clone()
		}
	}
}

/*pub struct GlobalDefPath {
	pub path: Vec<String>,
	pub crate_name: String,
	pub config_hash: u64
}*/

/*#[derive(Debug, PartialEq)]
pub struct DefIdWrapper(pub hir::def_id::DefId);
*/

impl CrateIdentifier {
	/// 
	/// @return a unique identifier containing the name and version of this crate
	/// that can be used as a filename for storing crate bundles.
	/// 
	pub fn get_filename(&self) -> String {
		self.name.clone() + "_"
			+ &self.version.0.to_string() + "_" 
			+ &self.version.1.to_string() + "_" 
			+ &self.version.2.to_string() + "-"
			+ &self.config_hash
	}
}

impl Crate {
	pub fn new(name: &str, version: (u64, u64, u64), config_hash: &str) -> Crate {
		Crate {
			metadata: CrateIdentifier {
				name: String::from(name),
				version: version,
				config_hash: config_hash.to_owned(),
			},

			mods: vec![],
			functions: vec![]
		}
	}

	/*fn find_mod_ids(&self, name: &str) -> Vec<usize> {
		let mut ids: Vec<usize> = Vec::new();
		for (module, id) in self.mods.iter().zip(0..self.mods.len()) {
			if module.name == name {
				ids.push(id);
			}
		}
		ids
	}*/

	/*pub fn get_mod_id(&self, path: &GlobalDefPath) -> Option<usize> {
		let mut current_mod: Option<usize> = None;
		for segment in path.0.data {
			let mods = self.find_mod_ids(&segment);
			if let Some(new_parent) = mods.into_iter().find(|x| self.mods[*x].parent_mod_id == current_mod) {
				current_mod = Some(new_parent)
			}
			else {
				return None
			}
		}
		current_mod
	}*/
	
	/*
	pub fn get_function(&self, def_id: hir::def_id::DefId) -> Option<&Function> {
		self.functions.iter().find(|f| f.def_id == DefIdWrapper(def_id) )
	}
	*/
}

/*
impl Serialize for DefIdWrapper {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_none()
	}
}

impl<'de> Deserialize<'de> for DefIdWrapper {
	fn deserialize<D>(deserializer: D) -> Result<DefIdWrapper, D::Error>
	where
		D: Deserializer<'de>,
	{
		Ok(DefIdWrapper::default())
		//deserializer.deserialize_i32(I32Visitor)
	}
}
*/

/*impl Serialize for GlobalDefPath {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_none()
	}
}

impl<'de> Deserialize<'de> for GlobalDefPath {
	fn deserialize<D>(deserializer: D) -> Result<GlobalDefPath, D::Error>
	where
		D: Deserializer<'de>,
	{
		//Ok(Self(DefPath::make()))
		Err(serde::de::Error::custom("not yet implemented"))

		//deserializer.deserialize_i32(I32Visitor)
	}
}
*/

impl Default for Function {
	fn default() -> Self {
		Function {
			name: "".to_owned(),
			is_unsafe: false,
			calls: vec![],
			containing_mod: None,
			//def_id: DefIdWrapper::default(),
		}
	}
}

/*
impl Default for DefIdWrapper {
	fn default() -> Self {
		DefIdWrapper(hir::def_id::DefId::local(hir::def_id::DefIndex::from_raw_u32(0)))
	}
}
*/

/*impl GlobalDefPath {
	/// create an identifier to a local item
	pub fn new(data: Vec<String>) -> GlobalDefPath {
		GlobalDefPath {
			path: data,
			crate_name: "".to_owned(),
			config_hash: 0
		}
	}

	/*
	pub fn from_path(path: &hir::Path) -> GlobalDefPath {
		let mut new = GlobalDefPath {
			path: vec![],
			crate_name: "".to_owned(),
			config_hash: 0
		};

		for seg in &path.segments {
			let ident = seg.ident;
			new.path.push(ident.to_string());
		}

		new
	}
	*/

	///
	/// @warning does not yet support disambiguators
	///
	pub fn from_def_path_of_mod(def_path: &DefPath) -> GlobalDefPath {
		let mut segments: Vec<String> = Vec::new();
		for elem in &def_path.data {
			if let DefPathData::Module(m) = elem.data {
				segments.push(m.as_str().to_string());
			}
		}
		return Self::new(segments);
	}

	pub fn remove_last_segment(mut self) -> Self {
		self.path.pop();
		self
	}
}*/

/*

impl From<DefId> for hir::def_id::DefId {
	fn from(def: DefId) -> Self {
		let indx: DefIndex = DefIndex::from_raw_u32(def.index);
		//hir::def_id::DefId::local(indx)
		hir::def_id::DefId{krate: hir::def_id::CrateNum::from_u32(def.crate_id), index: indx}
		/*{
			krate: hir::def_id::LOCAL_CRATE,
			index: DefIndex::from_u32(def.index)
		}*/
	}
}
*/
