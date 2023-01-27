use std::io::BufRead;

use serde::{ Deserialize, Deserializer, de };
use serde_json::Value;

fn de_amount< 'de, D : Deserializer< 'de > >( deserializer : D ) -> Result< f32, D::Error >
{
  Ok
  (
    match Value::deserialize( deserializer )?
    {
      Value::String( s ) => s.parse().map_err( de::Error::custom )?,
      Value::Number( num ) => num.as_f64().ok_or( de::Error::custom( "Invalid number" ) )? as f32,
      _ => return Err( de::Error::custom( "Invalid type" ) )
    }
  )
}

fn de_zip_code< 'de, D : Deserializer< 'de > >( deserializer : D ) -> Result< u32, D::Error >
{
  Ok
  (
    match Value::deserialize( deserializer )?
    {
      Value::String( s ) => s.parse().map_err( de::Error::custom )?,
      Value::Number( num ) => num.as_u64().ok_or( de::Error::custom( "Invalid number" ) )? as u32,
      _ => return Err( de::Error::custom( "Invalid type" ) )
    }
  )
}

#[ derive( Deserialize, PartialEq, Debug ) ]
pub struct CostAmount
{
  #[ serde( deserialize_with = "de_amount" ) ]
  pub amount : f32,
}

#[ derive( Deserialize, PartialEq, Debug ) ]
pub struct Cost
{
  #[ serde( rename = "totalAmount" ) ]
  pub total_amount : CostAmount,
}

#[ derive( Deserialize, PartialEq, Debug ) ]
pub struct DeliveryAddress
{
  pub city : String,
  #[ serde( rename = "countryCode" ) ]
  pub country_code : String,
  #[ serde( rename = "provinceCode" ) ]
  pub province_code : String,
  #[ serde( deserialize_with = "de_zip_code" ) ]
  pub zip : u32
}

#[ derive( Deserialize, PartialEq, Debug ) ]
pub struct DeliveryGroup
{
  #[ serde( rename = "deliveryAddress" ) ]
  pub delivery_address : DeliveryAddress
}

#[ derive( Deserialize, PartialEq, Debug ) ]
pub struct Customer
{
  pub email : String
}

#[ derive( Deserialize, PartialEq, Debug ) ]
pub struct Identity
{
  pub customer : Customer
}

#[ derive( Deserialize, PartialEq, Debug ) ]
pub struct Cart
{
  pub cost : Cost,
  #[ serde( rename = "deliveryGroups" ) ]
  pub delivery_groups : Vec< DeliveryGroup >,
  #[ serde( rename = "buyerIdentity" ) ]
  pub buyer_identity : Identity
}

#[ derive( Deserialize ) ]
pub struct Shop
{
  pub cart : Cart
}

pub fn parse_to_shop< R : BufRead >( reader : R ) -> Result< Shop, serde_json::Error >
{
  serde_json::from_reader( reader )
}

#[ cfg( test ) ]
mod tests
{
  use std::{ fs::File, io::BufReader };

  use super::*;

  #[ test ]
  fn parsing_produces_correct_structure_from_file()
  {
    let file = File::open( "./input.json" ).expect( "Failed to open the file" );
    let reader = BufReader::new( file );
    let shop = parse_to_shop( reader ).expect( "Failed to parse the file" );

    assert_eq!( shop.cart.cost.total_amount.amount, 123.0 );
    assert_eq!( shop.cart.delivery_groups.len(), 1 );
    assert_eq!( shop.cart.buyer_identity.customer.email, "helo@gmail.com" );
  }

  #[ test ]
  fn parsing_hardcoded_json()
  {
    let json = serde_json::json!
    {
      {
        "cart" :
        {
          "cost" : { "totalAmount" : { "amount" : "123.0" } },
          "deliveryGroups" :
          [
            {
              "deliveryAddress" :
              {
                "city" : "Sr",
                "countryCode" : "IN",
                "provinceCode" : "GJ",
                "zip" : "123123"
              }
            }
          ],
          "buyerIdentity" :
          {
            "customer" : { "email": "helo@gmail.com" }
          }	
        }
      }
    }.to_string();

    let shop = parse_to_shop( json.as_bytes() ).expect( "Failed to parse hardcoded json." );

    assert_eq!( shop.cart.cost.total_amount.amount, 123.0 );
    assert_eq!( shop.cart.delivery_groups.len(), 1 );
    assert_eq!( shop.cart.buyer_identity.customer.email, "helo@gmail.com" );
  }
}
