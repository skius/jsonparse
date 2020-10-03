use jsonparse::{Parser, OptionValueExt};

fn main() {
    let json_str = r#"
    {
        "no": false,
        "inner_obj": {
            "inner_field": null,
            "inner_array_of_objects": [
                {
                    "in_obj_1_a": true,
                    "in_obj_1_b": 32.12345
                },
                {
                    "in_obj_2_a": [2,[3]]
                }
            ]
        },
        "some_number": 32,
        "array_thingy": [
            2, 3, "noooo"
        ],
        "a_string": "my_string",
        "test_string": "no"
    }
    "#;

    // Version with Option<&Value> sugar, returns None if index not found:

    let p = Parser::new(json_str);
    let val = p.parse();

    println!("{:?}", val.as_ref().get_map("inner_obj").get_map("inner_array_of_objects").get_arr(1));


    // Version with Index, panics if index not found:

    let p = Parser::new(json_str);
    let val = p.parse().unwrap();

    println!("{:?}", val["inner_obj"]["inner_array_of_objects"][1]);
}