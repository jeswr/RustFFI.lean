extern crate oxrdf;
extern crate oxttl;

use oxrdf::{NamedNodeRef, vocab::rdf};
use oxttl::TurtleParser;

#[no_mangle]
pub extern "C" fn add_from_rust(a : i32, b : i32) -> i32 {

let file = b"@base <http://example.com/> .
@prefix schema: <http://schema.org/> .
<foo> a schema:Person ;
    schema:name \"Foo\" .
<bar> a schema:Person ;
    schema:name \"Bar\" .";

let schema_person = NamedNodeRef::new("http://schema.org/Person").unwrap();
let mut count = 0;
for triple in TurtleParser::new().parse_read(file.as_ref()) {
    let triple = triple.unwrap();
    if triple.predicate == rdf::TYPE && triple.object == schema_person.into() {
        count += 1;
    }
}

print!("Found {} schema:Person triples\n", schema_person.as_str());
// assert_eq!(2, count);

    return a + b + 5 + count
}
