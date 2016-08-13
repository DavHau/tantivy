/*!


# Schema definition

Tantivy has a very strict schema.
The schema defines information about the fields your index contains, that is for each field :

* the field name (may only contain letters `[a-zA-Z]`, number `[0-9]`, and `_`)
* the type of the field (currently only  `text` and `u32` are supported)
* how the field should be indexed / stored.
    
This very last point is critical as it will enable / disable some of the functionality
for your index.

Tantivy's schema is stored within the `meta.json` file at the root of your
directory.



# Building a schema "programmatically"


## Setting a text field

### Example

```
use tantivy::schema::*;
let mut schema = Schema::new();
let title_options = TextOptions::new()
    .set_stored()
    .set_indexing_options(TextIndexingOptions::TokenizedWithFreqAndPosition);
schema.add_text_field("title_options", title_options);
```

We can split the problem of generating a search result page into two phases :

* identifying the list of 10 or so document to be displayed (Conceptually `query -> doc_ids[]`)
* for each of these documents, retrieving the information required to generate the serp page. (`doc_ids[] -> Document[]`)

In the first phase, the hability to search for documents by the given field, is determined by the [`TextIndexingOptions`](enum.TextIndexingOptions.html) of our
[`TextOptions`](struct.TextOptions.html).

The effect of each possible settings is described more in detail [`TextIndexingOptions`](enum.TextIndexingOptions.html).

On the other hand setting the field as stored or not determines whether the field should be returned when [`searcher.doc(doc_address)`](../struct.Searcher.html#method.doc)
is called.

### Shortcuts

For convenience, a few special value of `TextOptions` for your convenience.
They can be composed using the `|` operator.
The example can be rewritten :


```
use tantivy::schema::*;
let mut schema = Schema::new();
schema.add_text_field("title_options", TEXT | STORED);
``` 



## Setting a u32 field

### Example

```
use tantivy::schema::*;
let mut schema = Schema::new();
let num_stars_options = U32Options::new()
    .set_stored()
    .set_indexed();
schema.add_u32_field("num_stars", num_stars_options);
```

Just like for Text fields (see above),
setting the field as stored defines whether the field will be
returned when [`searcher.doc(doc_address)`](../struct.Searcher.html#method.doc) is called, 
and setting the field as indexed means that we will be able perform queries such as `num_stars:10`.
Note that contrary to text fields, u32 can only be indexed in one way for the moment. 
This may change when we will start supporting range queries.

The `fast` option on the other hand is specific to u32 fields, and is only relevant 
if you are implementing your own queries. This functionality is somewhat similar to Lucene's 
`DocValues`.

u32 that are indexed as fast will be stored in a special data structure that will
make it possible to access the u32 value given the doc id rapidly. This is useful if the value of
the field is required during scoring or collection for instance.

*/ 

mod schema;
mod term;
mod document;

mod field_entry;
mod field_value;

mod text_options;
mod u32_options;
mod field;


pub use self::schema::Schema;
pub use self::document::Document;
pub use self::field::Field;
pub use self::term::Term;

pub use self::field_entry::FieldEntry;
pub use self::field_value::FieldValue;
pub use self::field_entry::FieldType;

pub use self::text_options::TextOptions;
pub use self::text_options::TEXT;
pub use self::text_options::STRING;
pub use self::text_options::STORED;
pub use self::text_options::TextIndexingOptions;

pub use self::u32_options::U32Options;
pub use self::u32_options::FAST;

use regex::Regex;

pub fn is_valid_field_name(field_name: &str) -> bool {
    lazy_static! {
        static ref FIELD_NAME_PTN: Regex = Regex::new("[_a-zA-Z0-9]+").unwrap();
    }
    FIELD_NAME_PTN.is_match(field_name)
}


