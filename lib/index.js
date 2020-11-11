var addon = require('../native');

////Publish

// let result = addon.iotapublish({
//     //network
//     min_weight_magnitude: 9,
//     local_pow: true,
//     //channel
//     multi_branching_flag: false,
//     network_url: "https://nodes.devnet.iota.org:443",
//     author_secret: "MY_SECRET_AUTHOR2",
//     payload: "see monsieur, here comes the name message"
// });

////Subscribe

let result = addon.iotasubscribe({
    //network
    min_weight_magnitude: 9,
    local_pow: true,
    //channel
    network_url: "https://nodes.devnet.iota.org:443",
    subscriber_secret: "MY_SECRET_SUBSCRIBER",
    channel_address: "b1ca95ecd5511e895c06d503850308a4257723406b4a2101f0264f0b6c2e206b0000000000000000", 
    announce_message_identifier: "6d9ff598a8ed7c1d2f3fe06f", 
    signed_message_identifier: "28588f41668737b02163559f"
});

console.log(result);

function converter(sentence){
    let capitalized = []
    let words = sentence.split(" ") //split the sentence into words
    words.forEach(word => { 
        let capitalizedWord = word.slice(0, 1).toUpperCase() + word.slice(1) //capitalize the first letter of every word
        capitalized.push(capitalizedWord)         
    })
    let converted = capitalized.join(" ") 
    return converted
}
module.exports = {
    iotasubscribe: function() {},
    otherMethod: function() {},
};
