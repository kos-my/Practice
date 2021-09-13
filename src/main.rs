extern crate serde;
extern crate serde_json;
extern crate serde_derive;

use std::fs;
use std::env;
use std::process;
use std::collections::HashMap;
use std::fmt;
use serde_derive::{ Serialize, Deserialize };

//

fn main()
{
  let args : Vec<String> = env::args().collect();
  let len = args.len();
  verify_args_len( len );

  let file = args[ 1 ].clone();
  let read = fs::File::open( file ).unwrap_or_else
  (
    | err |
    {
      eprintln!( "{}", err );
      process::exit( -1 );
    }
  );
  let json : Schema = serde_json::from_reader( read ).expect( "File should be proper JSON." );

  for object in &json.objects
  {
    println!( "{}", object );
  }

  let ( pairs, areas ) = pair_intersections_get( &json.objects );
  for i in 0..areas.len()
  {
    println!( "Rectangle {} intersects with {}. Area of intersection : {}", pairs[ i * 2 ], pairs[ i * 2 + 1 ], areas[ i ] );
  }
}

//

/// Struct Rect represents rectangle object structure.
#[ derive( Debug, Serialize, Deserialize ) ]
struct Rect
{
  name : String,
  properties : Option<Vec<HashMap<String, String>>>,
  width : f64,
  height : f64,
  x : f64,
  y:f64,
}

//

impl Rect
{
  /// Method area() calculates area of rectangle.
  ///
  /// # Example
  /// ```
  /// let r = Rect { name : String::from( "A" ), properties : None, width : 1.0, height : 1.0, x : 0.0, y : 0.0 };
  /// let area = r.area();
  /// assert_eq!( area, 1.0 );
  /// ```
  fn area( &self ) -> f64
  {
    self.width * self.height
  }

  /// Method intersects_with() checks of intersection with another rectangle.
  ///
  /// # Example
  /// ```
  /// let r1 = Rect { name : String::from( "A" ), properties : None, width : 1.0, height : 1.0, x : 0.0, y : 0.0 };
  /// let r2 = Rect { name : String::from( "B" ), properties : None, width : 1.0, height : 1.0, x : 0.5, y : 0.5 };
  /// let intersected = r1.intersects_with( &r2 );
  /// assert_eq!( area, true );
  ///
  /// let r1 = Rect { name : String::from( "A" ), properties : None, width : 1.0, height : 1.0, x : 0.0, y : 0.0 };
  /// let r2 = Rect { name : String::from( "B" ), properties : None, width : 1.0, height : 1.0, x : 2.0, y : 2.0 };
  /// let intersected = r1.intersects_with( &r2 );
  /// assert_eq!( area, false );
  /// ```
  fn intersects_with( &self, src : &Rect ) -> bool
  {
    ( ( self.x >= src.x && self.x <= src.x + src.width ) || ( self.x <= src.x && self.x + self.width >= src.x ) )
    && ( ( self.y >= src.y && self.y <= src.y + src.height ) || ( self.y <= src.y && self.y + self.height >= src.y ) )
  }

  /// Method intersection_area_get() calculates area of intersection. If no intersection exists,
  /// then method returns -1.
  ///
  /// # Example
  /// ```
  /// let r1 = Rect { name : String::from( "A" ), properties : None, width : 1.0, height : 1.0, x : 0.0, y : 0.0 };
  /// let r2 = Rect { name : String::from( "B" ), properties : None, width : 1.0, height : 1.0, x : 0.5, y : 0.5 };
  /// let intersected = r1.intersection_area_get( &r2 );
  /// assert_eq!( area, 0.25 );
  ///
  /// let r1 = Rect { name : String::from( "A" ), properties : None, width : 1.0, height : 1.0, x : 0.0, y : 0.0 };
  /// let r2 = Rect { name : String::from( "B" ), properties : None, width : 1.0, height : 1.0, x : 2.0, y : 2.0 };
  /// let intersected = r1.intersection_area_get( &r2 );
  /// assert_eq!( area, -1.0 );
  /// ```
  fn intersection_area_get( &self, src : &Rect ) -> f64
  {
    if !self.intersects_with( src )
    {
      return -1.0;
    }

    let dx : f64;
    let dy : f64;

    if self.x < src.x
    {
      if ( self.x + self.width ) < ( src.x + src.width )
      {
        dx = self.x + self.width - src.x;
      }
      else
      {
        dx = src.width;
      }
    }
    else
    {
      if ( self.x + self.width ) < ( src.x + src.width )
      {
        dx = self.width;
      }
      else
      {
        dx = src.x + src.width - self.x;
      }
    }

    if self.y < src.y
    {
      if ( self.y + self.height ) < ( src.y + src.height )
      {
        dy = self.y + self.height - src.y;
      }
      else
      {
        dy = src.height;
      }
    }
    else
    {
      if ( self.y + self.height ) < ( src.y + src.height )
      {
        dy = self.height;
      }
      else
      {
        dy = src.y + src.height - self.y;
      }
    }

    dx * dy
  }
}

impl fmt::Display for Rect
{
  fn fmt( &self, f : &mut fmt::Formatter ) -> fmt::Result
  {
    write!
    (
      f, "Name : {}\nPosition :\n  x : {}, y : {}\nDimensions :\n  width : {}, height : {}\nArea : {}\n",
      self.name.as_str(),
      &self.x.to_string(), &self.y.to_string(),
      &self.width.to_string(), &self.height.to_string(), &self.area()
    )
  }
}

/// Routine pair_intersections_get() finds pairs of rectangles that intersect each other and
/// calculates the area of intersection.
///
/// The returned tuple of vectors contains the vector of pairs
/// and vector of areas. Each pair of elements in vector of pairs define pair of rectangles that
/// intersect each other.
///
/// # Example
/// ```
/// let r1 = Rect { name : String::from( "A" ), properties : None, width : 1.0, height : 1.0, x : 0.0, y : 0.0 };
/// let r2 = Rect { name : String::from( "B" ), properties : None, width : 1.0, height : 1.0, x : 0.5, y : 0.5 };
/// let r3 = Rect { name : String::from( "C" ), properties : None, width : 1.0, height : 1.0, x : 2.0, y : 2.0 };
/// let vec = vec![ r1, r2, r3 ];
/// let ( pairs, areas ) = pair_intersections_get( &vec );
/// assert_eq!( pairs.len(), 2 );
/// assert_eq!( pairs[ 0 ], "A" );
/// assert_eq!( pairs[ 1 ], "B" );
/// assert_eq!( areas.len(), 1 );
/// assert_eq!( areas[ 0 ], 0.25 );
/// ```
fn pair_intersections_get( src : &Vec<Rect>) -> (Vec<String>, Vec<f64> )
{
  let mut pairs = vec![];
  let mut areas = vec![];
  let len = src.len();
  for i in 0..len
  {
    for j in i + 1..len
    {
      if src[ i ].intersects_with( &src[ j ] )
      {
        pairs.push( src[ i ].name.clone() );
        pairs.push( src[ j ].name.clone() );
        areas.push( src[ i ].intersection_area_get( &src[ j ] ) );
      }
    }
  }
  ( pairs, areas )
}

/// Struct Schema represents standard JSON document structure.
#[ derive( Serialize, Deserialize ) ]
struct Schema
{
  draworder : String,
  id : usize,
  name : String,
  objects : Vec<Rect>,
  opacity : usize,
  visible : bool,
}

//

fn verify_args_len( len : usize )
{
  if len < 2
  {
    eprintln!( "Not enough arguments. Expects path to JSON file." );
    process::exit( -1 );
  }
  else if len > 2
  {
    eprintln!( "Expects single path as parameter." );
    process::exit( -1 );
  }
}

//

#[cfg(test)]
#[path = "../test/test.rs"]
mod test;
