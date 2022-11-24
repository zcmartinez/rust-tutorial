// leer los archivos previous: ml_data, current: ml_data
mod ml_data; 
use std::path::Path; 
use std::collections::HashMap; 
use std::vec::Vec; 
use rstar::RTree; 

fn main(){
	
	let path_previous = Path::new("/home/zaira/Documentos/programacion_algoritmos/tareaJsonFile/rust-tutorial-master/resources/1663154348643_8ZGUJJLLWV.json");
	let path_current = Path::new("/home/zaira/Documentos/programacion_algoritmos/tareaJsonFile/rust-tutorial-master/resources/1663154348643_8ZGUJJLLWV.json");

	let nodes_path = ml_data::read_ml_json(path_previous).element_statistics.nodes; 
	let nodes_current = ml_data::read_ml_json(path_current).element_statistics.nodes;

	//let cant_nodes = nodes_path.iter().count(); 
	//println!("{}", cant_nodes); 
	// print info nodes
	//println!("{:?}", nodes_path[0]); 
	
	
	// find index where xx == true 
	let cant_nodes = nodes_path.iter().count(); 
	let cant_nodes_current = nodes_path.iter().count(); 
	let mut indice = 0; 
	for i in 0..cant_nodes{
		let f = nodes_path[i].a.get("XX");
		if f != None{ 
			indice = i; 
		}
	}
	
	// delete keys and make hashmap == previous
	let mut previous = HashMap::new();
	for (key, value) in &nodes_path[indice].a{
		if key != "WH" || key != "HT" || key != "TP" || key != "XX"{
			//println!("{} {}", key, value); 
			previous.insert(key, value); 
		}
	}
	//println!("{:?}", previous);
	
	let c = previous.iter().count() as f32; 
	println!("{}", c); 
	// compare hashmaps and save in vector
	//println!("{:?}", previous.keys()); 
	let mut vec: Vec<f32>= Vec::new(); 
	for node in 0..cant_nodes_current{
		let mut count:f32 = 0.0; 
		for i in previous.keys(){
			for j in nodes_current[node].a.keys(){
				if i==&j{
					if previous.get(i) != None && nodes_current[node].a.get(j) != None{
					//println!("{:?}", nodes_current[3].a.get(j)); 
						if previous.get(i).unwrap() == &nodes_current[node].a.get(j).unwrap(){
							count += 1.0; 
						}
					}
				}
			} 
		}	
	vec.push(count/c); 
	}
	
	
	//println!("{:?}", nodes_current[indice]); 
	println!(" vector de correlacion: {:?}", vec); 
	
	//let maxvalue = *vec.iter().max().unwrap(); 
	//println!("{:?}", maxvalue); 
	 
} 




