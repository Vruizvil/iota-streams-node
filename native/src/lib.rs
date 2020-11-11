#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use neon::prelude::*;
use anyhow::{Result, bail};
use std::env;

use iota_streams::app_channels::api::tangle::{
    Author, Address, Transport, Subscriber
};

use neon::register_module;
use neon_serde::export;
use serde_derive::{Deserialize, Serialize};

mod iotasubscriber;
use crate::iotasubscriber::subscribe::get_signed_messages;
use crate::iotasubscriber::subscribe::get_announcement;
use crate::iotasubscriber::subscribe::get_keyload;
use crate::iotasubscriber::subscribe::create_message_link;
use crate::iotasubscriber::subscribe::subscribe;

mod iotapublisher;
use crate::iotapublisher::announce::start_a_new_channel;
use crate::iotapublisher::send_message::send_signed_message;
use crate::iotapublisher::get_subscribers::get_subscriptions_and_share_keyload;
use crate::iotapublisher::send_masked_payload::send_masked_payload;

use iota_streams::app::{
    transport::tangle::{
        PAYLOAD_BYTES,
        client:: {
            Client,
            SendTrytesOptions
        }
    }
};

use iota::client as iota_client;

use iota_conversion::trytes_converter::{
    bytes_to_trytes
};

#[derive(Serialize, Deserialize, Debug)]
struct PublishMessage {
    //network
    min_weight_magnitude: u8,
    local_pow: bool,
    //channel
    multi_branching_flag: bool,
    network_url: String,
    //author
    author_secret: String,
    payload: String
}

#[derive(Serialize, Deserialize, Debug)]
struct SubscribeMessage{
    //network
    min_weight_magnitude: u8,
    local_pow: bool,
    //channel
    network_url: String,
    //subscriber
    subscriber_secret: String,
    //published message
    channel_address: String, 
    announce_message_identifier: String, 
    signed_message_identifier: String
}

export! {
    fn sayHello(name: String) -> String {
        format!("Hello, {}!", name)
    }

    fn iotapublish(pubmsg: PublishMessage) -> String {

    //  -------- IOTA network settings ---------

    // Change the default settings to use a lower minimum weight magnitude for the Devnet
    let mut send_opt = SendTrytesOptions::default();
    // default is 14 in example its 9
    send_opt.min_weight_magnitude = pubmsg.min_weight_magnitude;
    send_opt.local_pow = pubmsg.local_pow;
    //let url = "https://nodes.devnet.iota.org:443"
    let url = &pubmsg.network_url;

    // Connect to an IOTA node
    let client: Client = Client::new(send_opt, iota_client::ClientBuilder::new().node(url).unwrap().build().unwrap());
    let encoding = "utf-8";

    // --------------- Author -------------------

    // Create a new channel
    // channel address would depend on author secret
    let mut author = Author::new(&pubmsg.author_secret, encoding, PAYLOAD_BYTES, pubmsg.multi_branching_flag, client);
    let channel_address = author.channel_address().unwrap().to_string();
    
    let announce_message = start_a_new_channel(&mut author).unwrap();
    let announce_msgid = announce_message.msgid.to_string();
    let signed_message = send_signed_message(&mut author, &channel_address, &announce_msgid, &pubmsg.payload.to_string()).unwrap();
        
    format!("{} | {} | {}", channel_address, announce_msgid, signed_message.msgid)
    }

    fn iotasubscribe(submsg: SubscribeMessage) -> String {
        // Change the default settings to use a lower minimum weight magnitude for the Devnet
        let mut send_opt = SendTrytesOptions::default();
        // default is 14
        send_opt.min_weight_magnitude = submsg.min_weight_magnitude;
        send_opt.local_pow = submsg.local_pow;
    
        let url =  &submsg.network_url;

        // Connect to an IOTA node
        let client: Client = Client::new(send_opt, iota_client::ClientBuilder::new().node(url).unwrap().build().unwrap());
    
        // Create a new subscriber
        let encoding = "utf-8";
        let mut subscriber = Subscriber::new(&submsg.subscriber_secret, encoding, PAYLOAD_BYTES, client);
    
        // let args: Vec<String> = env::args().collect();
    
        let channel_address = &submsg.channel_address;
        let announce_message_identifier = &submsg.announce_message_identifier;
        let signed_message_identifier = &submsg.signed_message_identifier;
    
        match get_announcement(&mut subscriber, &channel_address.to_string(), &announce_message_identifier.to_string()){
            Ok(()) => (),
            Err(error) => println!("Failed with error {}", error),
        }
    
        match get_signed_messages(&mut subscriber, &channel_address.to_string(), &signed_message_identifier.to_string()){
            Ok(res) => {format!("{}", res)},
            Err(error) => {println!("Failed with error {}", error); format!("{}", "signed_messaged")},
        }
     }
}