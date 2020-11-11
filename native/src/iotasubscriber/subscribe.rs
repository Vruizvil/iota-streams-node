#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota::client as iota_client;

use iota_streams::app_channels::api::tangle::{
    Address, Transport, Subscriber
};
use iota_streams::app::transport::tangle::{
    PAYLOAD_BYTES,
    client::{
        Client,
        SendTrytesOptions
    }
};

use anyhow::{Result, bail};
use std::env;

pub fn get_signed_messages<T: Transport>(subscriber: &mut Subscriber<T>, channel_address: &String, signed_message_identifier: &String) -> Result<String> {
        
    // Convert the channel address and message identifier to a link
    let message_link = match create_message_link(&channel_address, &signed_message_identifier){
        Ok(message_link) => message_link,
        Err(error) => bail!(error),
    };

    // First returned value is the senders public key. We wont be using that in this tutorial
    let (_, public_payload, masked_payload) = subscriber.receive_signed_packet(&message_link)?;
    println!("Found and verified message");
    let finalmsg = format!("public: {:?} | private: {:?}", 
        String::from_utf8(public_payload.0), String::from_utf8(masked_payload.0));
    Ok(finalmsg.to_string())
}

pub fn get_announcement<T: Transport>(subscriber: &mut Subscriber<T>, channel_address: &String, announce_message_identifier: &String) -> Result<()> {
    
    // Convert the channel address and message identifier to a link
    let announcement_link = match create_message_link(&channel_address, &announce_message_identifier){
        Ok(announcement_link) => announcement_link,
        Err(error) => bail!(error),
    };

    println!("Receiving announcement message");
    subscriber.receive_announcement(&announcement_link)?;

    Ok(())
}

pub fn get_keyload<T: Transport>(subscriber: &mut Subscriber<T>, channel_address: &String, keyload_message_identifier: &String) -> Result<()> {
    
    // Convert the channel address and message identifier to an Address link type
    let keyload_link = match create_message_link(&channel_address, &keyload_message_identifier){
        Ok(keyload_link) => keyload_link,
        Err(error) => bail!(error),
    };

    // Use the IOTA client to find transactions with the corresponding channel address and tag
    subscriber.receive_keyload(&keyload_link)?;
    Ok(())
}

pub fn create_message_link(channel_address: &String, message_identifier: &String) -> Result<Address> {
    // Convert the channel address and message identifier to a link
    let message_link = match Address::from_str(&channel_address, &message_identifier){
        Ok(message_link) => message_link,
        Err(()) => bail!("Failed to create Address from {}:{}", &channel_address, &message_identifier),
    };

    // Use the IOTA client to find transactions with the corresponding channel address and tag
    Ok(message_link)
}

pub fn subscribe<T: Transport>(subscriber: &mut Subscriber<T>, channel_address: &String, announce_message_identifier: &String) -> Result<()> {

     // Convert the channel address and message identifier to a link
     let announcement_link = create_message_link(&channel_address, &announce_message_identifier)?;
 
    println!("Subscribing to channel");

    // Send a `Subscribe` message and link it to the message identifier 
    //of the first valid `Announce` message that was found on the Tangle
    let subscription = subscriber.send_subscribe(&announcement_link)?;
    println!("Published `Subscribe` message");
    println!("Paste this `Subscribe` message identifier into your author's command prompt  {}", subscription.msgid);
    Ok(())
 }
