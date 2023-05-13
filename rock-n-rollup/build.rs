extern crate skeptic;

use skeptic::*;

fn main() {
    // Add all markdown files in directory "book/".
    let mdbook_files = markdown_files_of_directory("../doc/src");
    // Also add "README.md" to the list of files.
    generate_doc_tests(&mdbook_files);
}
