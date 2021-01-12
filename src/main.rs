use ::shrink::Shrink;

fn main() {
    let json = "
    {
        \"name\": \"James Bourdelon\",
        \"age\": 42,
        \"height\": 170.688,
        \"favorite_quotes\": {
            \"Walt Disney\": \"The way to get started is to quit talking and begin doing\",
            \"Anne Frank\": \"Whoever is happy will make others happy too\"
        },
        \"spoken_languages\": [
            \"English\",
            \"German\"
        ]
    }
    ";

    let shr = Shrink::from_json(String::from(json));
    println!("The length of json is {}", json.len());
    println!("The length of shrink is {}", shr.len());
}
