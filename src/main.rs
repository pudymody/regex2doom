extern crate regex_automata;

use std::collections::{HashMap, HashSet, VecDeque};
use std::vec::Vec;
use std::fmt;
use std::convert::TryInto;
use std::fs::File;
use std::io::{Write};
use std::io;
use std::cmp::max;
use std::error::Error;
use regex_automata::{DFA, dense::Builder};

struct Node {
	id: u64,

	is_start: bool,
	is_end: bool,

	x: u64,
	y: u64,

	origin_vertex: u64,
	origin_sidedef: u64,
	origin_sector: u64,

	edges: HashMap<String, u64>,
}

impl Node {
	fn width(&self) -> u64 {
		let count_edges = self.edges.len();
		return max( 64, (count_edges * 64 + (count_edges + 1) * 32).try_into().unwrap() );
	}

	fn vertex_len(&self) -> u64 {
		let count_edges = self.edges.len();
		return (4 + 4 * count_edges).try_into().unwrap();
	}

	fn sidedef_len(&self) -> u64 {
		let count_edges = self.edges.len();
		return (4 + 8 * count_edges).try_into().unwrap();
	}

	fn sector_len(&self) -> u64 {
		let count_edges = self.edges.len();
		return (1 + count_edges).try_into().unwrap();
	}
}

impl fmt::Display for Node {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let width = self.width();
		if self.is_start {
			write!(f, "thing{{
				x={};
				y={};
				type=1;
				flags=7;
				angle=90;
			}}
			", self.x + 32, self.y + 32)?;
		}

		write!(f, "thing{{
			x={};
			y={};
			type=14;
			flags=7;
			id={};
			skill1=true;
			skill2=true;
			skill3=true;
			skill4=true;
			skill5=true;
			single=true;
			dm=true;
			coop=true;
		}}
		", self.x + 32, self.y + 32, self.id )?;

		write!(f, "vertex{{
			x={};
			y={};
		}}
		", self.x, self.y)?;

		write!(f, "vertex{{
			x={};
			y={};
		}}
		", self.x + width, self.y)?;

		write!(f, "vertex{{
			x={};
			y={};
		}}
		", self.x, self.y + 160)?;

		write!(f, "vertex{{
			x={};
			y={};
		}}
		", self.x + width, self.y + 160)?;

		write!(f, "linedef{{
			v1={};
			v2={};
			sidefront={};
			blocking=true;
		}}
		", self.origin_vertex + 1, self.origin_vertex, self.origin_sidedef)?;

		write!(f, "linedef{{
			v1={};
			v2={};
			sidefront={};
			blocking=true;
		}}
		", self.origin_vertex, self.origin_vertex + 2, self.origin_sidedef + 1)?;

		write!(f, "linedef{{
			v1={};
			v2={};
			sidefront={};
			blocking=true;
		}}
		", self.origin_vertex + 3, self.origin_vertex + 1, self.origin_sidedef + 2)?;

		write!(f, "linedef{{
			v1={};
			v2={};
			sidefront={};
			blocking=true;
		}}
		", self.origin_vertex + 2, self.origin_vertex + 3, self.origin_sidedef + 3)?;

		write!(f, "sidedef{{
			sector={};
			texturemiddle=\"{}\";
		}}
		", self.origin_sector, ( if self.is_end { "AQF038" } else { "FLAT5_2" } ))?;

		write!(f, "sidedef{{
			sector={};
			texturemiddle=\"{}\";
		}}
		", self.origin_sector, ( if self.is_end { "AQF038" } else { "FLAT5_2" } ))?;

		write!(f, "sidedef{{
			sector={};
			texturemiddle=\"{}\";
		}}
		", self.origin_sector, ( if self.is_end { "AQF038" } else { "FLAT5_2" } ))?;

		write!(f, "sidedef{{
			sector={};
			texturemiddle=\"{}\";
		}}
		", self.origin_sector, ( if self.is_end { "AQF038" } else { "FLAT5_2" } ))?;

		write!(f, "sector{{
			texturefloor=\"FLOOR4_8\";
			textureceiling=\"CEIL3_5\";
			heightceiling=128;
		}}
		")?;

		let mut i = 1;
		for (key, val) in self.edges.iter() {
			let offset_vertex = 4*(i-1) + 4 + self.origin_vertex;
			let offset_x = self.x + 32 * i + 64 * (i-1);
			let offset_sidedef = 8*(i-1) + 4 + self.origin_sidedef;

			write!(f, "vertex{{
				x={};
				y={};
			}}
			", offset_x , self.y + 128)?;

			write!(f, "vertex{{
				x={};
				y={};
			}}
			", offset_x , self.y + 64)?;

			write!(f, "vertex{{
				x={};
				y={};
			}}
			", offset_x + 64 , self.y + 128)?;

			write!(f, "vertex{{
				x={};
				y={};
			}}
			", offset_x + 64, self.y + 64)?;

			write!(f, "linedef{{
				v1={};
				v2={};
				sidefront={};
				sideback={};
				twosided=true;
				playercross=true;
				special=70;
				arg0={};
				repeatspecial=true;
			}}
			", offset_vertex + 1, offset_vertex, offset_sidedef, offset_sidedef + 4, val)?;

			write!(f, "linedef{{
				v1={};
				v2={};
				sidefront={};
				sideback={};
				twosided=true;
				playercross=true;
				special=70;
				arg0={};
				repeatspecial=true;
			}}
			", offset_vertex + 2, offset_vertex + 3, offset_sidedef + 1, offset_sidedef + 5, val)?;

			write!(f, "linedef{{
				v1={};
				v2={};
				sidefront={};
				sideback={};
				twosided=true;
				playercross=true;
				special=70;
				arg0={};
				repeatspecial=true;
			}}
			", offset_vertex, offset_vertex + 2, offset_sidedef + 2, offset_sidedef + 6, val)?;

			write!(f, "linedef{{
				v1={};
				v2={};
				sidefront={};
				sideback={};
				twosided=true;
				playercross=true;
				special=70;
				arg0={};
				repeatspecial=true;
			}}
			", offset_vertex + 3, offset_vertex + 1, offset_sidedef + 3, offset_sidedef + 7, val)?;

			write!(f, "sidedef{{
				sector={0};
			}}
			sidedef{{
				sector={0};
			}}
			sidedef{{
				sector={0};
			}}
			sidedef{{
				sector={0};
			}}
			sidedef{{
				sector={1};
				texturebottom=\"CEIL3_5\";
			}}
			sidedef{{
				sector={1};
				texturebottom=\"CEIL3_5\";
			}}
			sidedef{{
				sector={1};
				texturebottom=\"CEIL3_5\";
			}}
			sidedef{{
				sector={1};
				texturebottom=\"CEIL3_5\";
			}}

			sector{{
				texturefloor=\"{2}\";
				textureceiling=\"CEIL3_5\";
				heightceiling=128;
				heightfloor=8;
				xpanningfloor={3};
			}}
			", self.origin_sector + i, self.origin_sector, key, offset_x % 64)?;

			/*
				offset_x % 64 because textures are aligned to 64 unit grid. So whenever it is in a non divisible number by 64,
				it needs to be offseted by that.
			*/

			i += 1;
		}

		Ok(())
	}
}

fn regex2graph(regex: &str) -> Vec<Node> {
	let dfa = Builder::new().case_insensitive(true).minimize(true).anchored(true).build( &regex ).unwrap();
	let dfa_start = dfa.start_state();

	let alphabet: HashSet<u8> = (0x30..0x39).chain(0x41..0x5a).collect();

	let mut nodes = Vec::new();

	let mut visited = HashSet::from([ dfa_start ]);
	let mut queue = VecDeque::from([ dfa_start ]);

	while queue.len() > 0 {
		let state_id = queue.pop_front().unwrap();

		let mut node = Node {
			x: 0,
			y: 0,

			// need to offset ids by 1 to not have id = 0
			id: (state_id + 1).try_into().unwrap(),

			is_start: state_id == dfa_start,
			is_end: dfa.is_match_state(state_id),

			origin_vertex: 0,
			origin_sidedef: 0,
			origin_sector: 0,

			edges: HashMap::new(),
		};

		if !dfa.is_dead_state(state_id) {
			for character in alphabet.iter() {
				let next_state = dfa.next_state(state_id, *character);

				// need to offset ids by 1 to not have id = 0
				node.edges.insert((*character as char).to_string(), (next_state + 1).try_into().unwrap());
				if !visited.contains(&next_state) {
					queue.push_back( next_state );
					visited.insert( next_state );
				}
			}
		}

		nodes.push( node );
	}

	return nodes;
}

fn graph2map(nodes: &mut Vec<Node>) -> String {
	let mut map = String::from("namespace=\"zdoom\";");
	let mut x = 0;
	let mut origin_vertex = 0;
	let mut origin_sidedef = 0;
	let mut origin_sector = 0;

	for mut node in nodes.into_iter() {
		node.x = x;
		node.origin_sector = origin_sector;
		node.origin_vertex = origin_vertex;
		node.origin_sidedef = origin_sidedef;

		map.push_str( &node.to_string() );

		x += node.width() + 32;
		origin_vertex += node.vertex_len();
		origin_sidedef += node.sidedef_len();
		origin_sector += node.sector_len();
	}

	return map;
}

fn write_map(map: &str) -> std::io::Result<()> {
	let mut file = File::create("MAP01.wad")?;
	file.write(b"PWAD")?;
	file.write(&[4,0,0,0])?;
	file.write(&[0xC,0,0,0])?;

	file.write(&[0,0,0,0])?; //location
	file.write(&[0,0,0,0])?; // size
	file.write(&[0x45,0x31,0x4D,0x31, 0, 0, 0, 0])?; //e1m1

	file.write(&[0x4c,0,0,0])?; //location
	file.write(&map.len().to_le_bytes()[0..4])?; // size
	file.write(&[0x54, 0x45, 0x58, 0x54, 0x4D, 0x41, 0x50, 0])?; //textmap

	file.write(&[0,0,0,0])?; //location
	file.write(&[0,0,0,0])?; // size
	file.write(&[0x42, 0x45, 0x48, 0x41, 0x56, 0x49, 0x4F, 0x52])?; //behavior

	file.write(&[0,0,0,0])?; //location
	file.write(&[0,0,0,0])?; // size
	file.write(&[0x45, 0x4E, 0x44, 0x4D, 0x41, 0x50, 0, 0])?; //endmap

	file.write(map.as_bytes())?;
	Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {

	let mut raw_regex = String::new();
	let stdin = io::stdin();
	stdin.read_line(&mut raw_regex)?;
	raw_regex.make_ascii_uppercase();

	let mut nodes = regex2graph( &raw_regex.trim() );
	let map = graph2map( &mut nodes );
	write_map( &map )?;

	Ok(())
}
