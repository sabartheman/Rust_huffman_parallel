extern crate time;
extern crate crossbeam;

use time::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::collections::HashMap;
use std::io::{Write,BufWriter};


///////////////////////////////////////////////////////////////////////////////////////////////
///////// Define statements, structs
////////////////////////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////
////since creating a key for a huffman code is usually a
////tree a Node is defined to create the building blocks for the tree
/////////////////////////////////////////////////////////////

struct Node{
    freq: i32,
    ch: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    //The Option tool allows for the assignment of value None
    //The Box tool pushes the data type to the heap
}


////////////////////////////////////////////////////////////////////////////////////////////////
///////// Functions
////////////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////
/////// reading an input text file
////////////////////////////////////

fn read_file() -> String {
    //reading from a File
    //In this case I used the first chapter of Ender's Game
    //as my example text
    let path = Path::new("test.txt");
    let display = path.display();
    let mut file = match File::open(&path){
        //a error statement to let the user know that the file probably isn't where they intended
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    s.push_str(" ");
    match file.read_to_string(&mut s){
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => println!("Read from file successful"),
    }
    s
}


fn write_encoded(output_encoded: &str){
    let file = File::create("encoded.txt").expect("Unable to create file\n");
    let mut file = BufWriter::new(file);
    file.write_all(output_encoded.as_bytes()).expect("Unable to write to file\n");
}


fn write_encoded_parallel(output_encoded: &str, file_name: &str){
    let file = File::create(file_name).expect("Unable to create file\n");
    let mut file = BufWriter::new(file);
    file.write_all(output_encoded.as_bytes()).expect("Unable to write to file\n");
}

fn write_decoded(output_decoded: &str){
    let file = File::create("decoded.txt").expect("unable to create file\n");
    let mut file = BufWriter::new(file);
    file.write_all(output_decoded.as_bytes()).expect("Unable to write to file\n");
}

//setting up a function to count the frequency of characters
//This function will spit out the result in a hashmap.
fn frequency(s: &str) -> HashMap<char, i32> {
    let mut h = HashMap::new();
    for ch in s.chars() {
        let counter = h.entry(ch).or_insert(0);
        *counter += 1;
    }
    h
}


fn assign_codes(p: &Box<Node>,
                h: &mut HashMap<char, String>,
                s: String ) {

    if let Some(ch) = p.ch {
        h.insert(ch, s);
    } else {
        if let Some(ref l) = p.left {
            assign_codes(l, h, (s.clone() + "0"));
        }
        if let Some(ref r) = p.right {
            assign_codes(r, h, (s.clone() + "1"));
        }
    }
}


////////////////////////////
//// encryption process
////////////////////////////

//once the tree is built we can encode the string to output
//this is one of the final steps
//this can be parallized pretty easily
fn encode_string(s: &str, h: &HashMap<char, String>) ->String {
    let mut r = "".to_string();
    let mut t: Option<&String>;

    for ch in s.chars() {
        t = h.get(&ch);
        r.push_str(t.unwrap());
    }
    r
}

//This is the final step.  After the string is encoded we need to
//decode it to make sure that the whole process works correctly
//This might not be able to be parallized

fn decode_string(s: &str, root: &Box<Node>) -> String {
    let mut retval = "".to_string();
    let mut nodeptr = root;

    for x in s.chars(){
        if x == '0' {
            if let Some(ref l) = nodeptr.left {
                nodeptr = l;
            }
        }else {
            if let Some(ref r) = nodeptr.right{
                nodeptr = r;
            }
        }
        if let Some(ch) = nodeptr.ch {
            retval.push(ch);
            nodeptr = root;
        }
    }
    retval
}




///////////////////////////////////
//////// Data structure template
///////////////////////////////////


//initializing a new node that has null values for its children
//
fn new_node(freq: i32, ch_in: Option<char>) -> Node {
    Node {
        freq: freq, ch: ch_in,
        left: None, right: None,
    }
}

fn new_box(n: Node) -> Box<Node> {
    Box::new(n)
}

///////////////////////////////////
//// creating vectors for parallel
///////////////////////////////////
fn create_vector4() -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let string1 = String::from("");
    let string2 = String::from("");
    let string3 = String::from("");
    let string4 = String::from("");
    vec.push(string1);
    vec.push(string2);
    vec.push(string3);
    vec.push(string4);
    vec
}

fn create_vector3() -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let string1 = String::from("");
    let string2 = String::from("");
    let string3 = String::from("");
    vec.push(string1);
    vec.push(string2);
    vec.push(string3);
    vec
}

fn create_vector2() -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let string1 = String::from("");
    let string2 = String::from("");
    vec.push(string1);
    vec.push(string2);
    vec
}

fn create_vector1() -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let string1 = String::from("");
    vec.push(string1);
    vec
}





fn main() {

    let t1 = precise_time_s();
    //let input_string = String::from("abaabbbcdd");
    let input_string = read_file();
    //println!("Testing out frequency function on the String {}", input_string);
    println!("Amount of characters in file: {}",&input_string.len());

    //////////////////////////////////////////////////////
    //// Printing out the length for testing slicing
    //////////////////////////////////////////////////////
    //println!("The length of the string is {}", input_string.len());

    let h = frequency(&input_string);
    //println!("\n \nFrequency of characters:\n{:?}",h);

    ///////////////////////////////////////////////////////
    //// Process to create the key (hashcode tree)
    //// I don't think I can parallize this part
    ///////////////////////////////////////////////////////

    let mut p:Vec<Box<Node>> =
              h.iter()
              .map(|x| new_box(new_node(*(x.1), Some(*(x.0)))))
              .collect();

    while p.len() > 1 {
              p.sort_by(|a, b| (&(b.freq)).cmp(&(a.freq)));
              let a = p.pop().unwrap();
              let b = p.pop().unwrap();
              let mut c = new_box(new_node(a.freq + b.freq, None));
              c.left = Some(a);
              c.right = Some(b);
              p.push(c);
    }

    let root = p.pop().unwrap();

    let mut h:HashMap<char, String> = HashMap::new();

    //////////////////////////////////////////////////////
    ///// Might be able to parallize this part of the algorithm
    //////////////////////////////////////////////////////
    assign_codes(&root, &mut h, "".to_string());


    //////////////////////////////////////////////////////
    //// Should be able to parallize this next section
    //// Since the string can be split up into sections
    //////////////////////////////////////////////////////

    let s2 = precise_time_s();
    let en_message = encode_string(&input_string, &h);
    write_encoded(&en_message);
    let t2 = precise_time_s();


    //Setting up the vectors to store the induvidual data infection
    //each of these are different lengths for the number of threads going to around
    let mut encoded_chunks4 = create_vector4();
    let mut encoded_chunks3 = create_vector3();
    let mut encoded_chunks2 = create_vector2();
    let mut encoded_chunks1 = create_vector1();

    let mut parallel_string1 = String::from("");
    let mut parallel_string2 = String::from("");
    let mut parallel_string3 = String::from("");
    let mut parallel_string4 = String::from("");


    //initiallizing variables for timing each of the possible threading options

    let mut tp1_start  = precise_time_s();
    let mut tp1_finish = precise_time_s();
    let mut tp2_start  = precise_time_s();
    let mut tp2_finish = precise_time_s();
    let mut tp3_start  = precise_time_s();
    let mut tp3_finish = precise_time_s();
    let mut tp4_start  = precise_time_s();
    let mut tp4_finish = precise_time_s();


    if (input_string.len() % 4) == 0 {
        println!("\n \ncan run 4 threads");
        tp4_start = precise_time_s();
        crossbeam::scope(|scope| {
            //set a "counter"
            let mut i = 1;
            for elem in &mut encoded_chunks4 {
                //Need to clone data for each thread, otherwise the data will be moved inside
                //of thread because of ownership, thus ending their life when scope ends
                let input_temp = input_string.clone();
                let h_temp     = h.clone();

                    //followed the basic setup of the crossbeam example on their api page
                    scope.spawn(move|| {
                        //println!("hello {:?}, {}", &input_temp, &i);
                        *elem = encode_string(&input_temp[(((&i-1)*&input_temp.len())/4)+1..((&i*&input_temp.len())/4)], &h_temp);
                        i = i + 1;
                    });
            };


        });

        //because of crossbeam's scoped threads, the parent scope will not exit till the child
        //scoped threads are complete.  Dont need a .join function because of this.
        for i in &mut encoded_chunks4 {
            parallel_string4.push_str(&i);
        }
        let filename = String::from("encoded_thread4.txt");

        write_encoded_parallel(&parallel_string4, &filename);
        tp4_finish = precise_time_s();
    }

    if (input_string.len() % 3) == 0 {
        println!("\n \ncan run 3 threads");
        tp3_start = precise_time_s();
        crossbeam::scope(|scope| {
            let mut i = 1;
            for elem in &mut encoded_chunks3 {
                let input_temp = input_string.clone();
                let h_temp     = h.clone();

                    scope.spawn(move|| {
                        //println!("hello {:?}, {}", &input_temp, &i);
                        *elem = encode_string(&input_temp[(((&i-1)*&input_temp.len())/3)+1..((&i*&input_temp.len())/3)], &h_temp);
                        i = i + 1;
                    });
            };


        });

        //because of crossbeam
        for i in &mut encoded_chunks3 {
            parallel_string3.push_str(&i);
        }
        let filename = String::from("encoded_thread3.txt");

        write_encoded_parallel(&parallel_string3, &filename);
        tp3_finish = precise_time_s();
    }

    if (input_string.len() % 2) == 0 {
        println!("\n \ncan run 2 threads");
        tp2_start = precise_time_s();
        crossbeam::scope(|scope| {
            let mut i = 1;
            for elem in &mut encoded_chunks2 {
                let input_temp = input_string.clone();
                let h_temp     = h.clone();

                    scope.spawn(move|| {
                        //println!("hello {:?}, {}", &input_temp, &i);
                        *elem = encode_string(&input_temp[(((&i-1)*&input_temp.len())/2)+1..((&i*&input_temp.len())/2)], &h_temp);
                        i = i + 1;
                    });
            };


        });

        //because of crossbeam
        for i in &mut encoded_chunks2 {
            parallel_string2.push_str(&i);
        }
        let filename = String::from("encoded_thread2.txt");

        write_encoded_parallel(&parallel_string2, &filename);
        tp2_finish = precise_time_s();
    }

    if(input_string.len() %1) == 0 {
        println!("\n \ncan run 1 thread");
        tp1_start = precise_time_s();
        crossbeam::scope(|scope| {
            let mut i = 1;
            for elem in &mut encoded_chunks1 {
                let input_temp = input_string.clone();
                let h_temp     = h.clone();

                    scope.spawn(move|| {
                        //println!("hello {:?}, {}", &input_temp, &i);
                        *elem = encode_string(&input_temp[(((&i-1)*&input_temp.len())/1)+1..((&i*&input_temp.len())/1)], &h_temp);
                        i = i + 1;
                    });
            };


        });

        //because of crossbeam
        for i in &mut encoded_chunks1 {
            parallel_string1.push_str(&i);
        }

        let filename = String::from("encoded_thread1.txt");

        write_encoded_parallel(&parallel_string1, &filename);
        tp1_finish = precise_time_s();
    }


    //////////////////////////////////////////////////////
    //// Probably cannot parallize this next section
    //// Due to the uncertain nature of the encoded string
    //////////////////////////////////////////////////////

    //println!("\n \nTesting the encoding process of this code:\n{:?}", en_message);

    let s3 = precise_time_s();
    let de_message = decode_string(&en_message, &root);
    write_decoded(&de_message);
    let t3 = precise_time_s();

    /*
    let mut encoded_vector: Vec<String> = Vec::new();
    encoded_vector.push(parallel_string1);
    encoded_vector.push(parallel_string2);
    encoded_vector.push(parallel_string3);
    encoded_vector.push(parallel_string4);

    crossbeam::scope(|scope| {
        let mut i = 1;
        for elem in &mut encoded_chunks1 {
            let input_temp = input_string.clone();
            let h_temp     = h.clone();

                scope.spawn(move|| {
                    //println!("hello {:?}, {}", &input_temp, &i);
                    *elem = encode_string(&input_temp[((&i-1)*&input_temp.len())/1..((&i*&input_temp.len())/1)], &h_temp);
                    i = i + 1;
                });
        };
    });
    */

    //println!("\n \nTesting decoding method:\n{:?}", de_message);


    println!("\n \nTime to complete encoding from start: {}[s]", t2 - t1);
    println!("\n \nTime to complete decoding from start: {}[s]", t3 - t1);

    println!("\n \nTime to complete only the encoding process: {}[s]", t2 - s2);
    println!("\n \nTime to complete only the decoding process: {}[s]", t3 - s3);

    if (input_string.len() % 4) == 0{
        println!("\n \nTime to complete Parallel encoding process (4 threads): {}[s]", tp4_finish - tp4_start);
    }
    if (input_string.len() % 3) == 0{
        println!("\n \nTime to complete Parallel encoding process (3 threads): {}[s]", tp3_finish - tp3_start);
    }
    if (input_string.len() % 2) == 0{
        println!("\n \nTime to complete Parallel encoding process (2 threads): {}[s]", tp2_finish - tp2_start);
    }
    if (input_string.len() % 1) == 0{
        println!("\n \nTime to complete Parallel encoding process (1 thread) : {}[s]\nJust a single thread so not truly parallel", tp1_finish - tp1_start);
    }

    //need to spend time on porting this data to a csv to output some statistics.
    //need to run script to get multiple test points to average data.

    ///////////////////////////////////////////////////////////////////////////
    ////// Writing data to a csv to output benchmarks
    ////////////////////////////////////////////////////////////////////////////

    let mut file =
        OpenOptions::new()
        .write(true)
        .append(true)
        .open("data_times.csv")
        .unwrap();

    if let Err(e) = writeln!(file, "{},{},{},{},{},{},{},", tp1_finish - tp1_start, tp2_finish - tp2_start, tp3_finish - tp3_start, tp4_finish - tp4_start, t3-t1, t2-s2, t3-s3) {
        println!("{},{},{},{},", tp1_finish - tp1_start, tp2_finish - tp2_start, tp3_finish - tp3_start, tp4_finish - tp4_start);
    }

}
