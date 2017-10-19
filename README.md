# just 
# Just-Another-Json-Tool

### What does it do

This tool takes an input stream, STDIN or a file, verifies that it is correct json, and then prints it to STDOUT or a file.  

### Why 

I needed a `json_xs` type tool to pretty print some json. It was a simple enough tool, I thought "Hey! why don't I just make another one". I'm using the [json crate](https://crates.io/crates/json).

### Example

    russell$ cat example.json
    {"title": "Some json","content": "Look at this content!"}

    russell$ cat example.json | just
    {
        "title": "Some json",
        "content": "Look at this content!"
    }%
