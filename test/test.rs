use super::*;

fn rect_make( name : String, x : f64, y : f64, width : f64, height : f64 ) -> Rect
{
  Rect
  {
    name,
    properties : None,
    width,
    height,
    x,
    y,
  }
}

#[test]
fn intersects_identical()
{
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.0, 0.0, 1.0, 1.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), true );
}
#[test]
fn intersects_inside()
{
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.25, 0.25, 0.5, 0.5 );
  assert_eq!( rect1.intersects_with( &rect2 ), true );
}
#[test]
fn intersects_outside()
{
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.5, -0.5, 1.0, 2.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), true );
}
#[test]
fn intersects_on_right()
{
  println!( "partial" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.5, 0.0, 1.0, 1.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), true );

  println!( "intersects on side" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 1.0, 0.0, 1.0, 1.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), true );
}
#[test]
fn intersects_on_left()
{
  println!( "partial" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), -0.5, 0.0, 1.0, 1.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), true );

  println!( "intersects on side" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), -1.0, 0.0, 1.0, 1.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), true );
}
#[test]
fn intersects_on_top()
{
  println!( "partial" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.0, 0.5, 1.0, 1.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), true );

  println!( "intersects on side" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.0, 1.0, 1.0, 1.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), true );
}
#[test]
fn intersects_on_bottom()
{
  println!( "partial" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.0, -0.5, 1.0, 1.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), true );

  println!( "intersects on side" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.0, -1.0, 1.0, 1.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), true );
}
#[test]
fn intersects_only_dot()
{
  println!( "right-bottom" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 1.0, -1.0, 1.0, 1.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), true );

  println!( "right-top" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 1.0, 1.0, 1.0, 1.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), true );

  println!( "left-bottom" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), -1.0, -1.0, 1.0, 1.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), true );

  println!( "left-top" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), -1.0, 1.0, 1.0, 1.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), true );
}
#[test]
fn intersects_no_rect()
{
  println!( "x is bigger" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 1.1, -1.0, 1.0, 1.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), false );

  println!( "y is bigger" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 1.0, 1.1, 1.0, 1.0 );
  assert_eq!( rect1.intersects_with( &rect2 ), false );
}

//

#[test]
fn area_of_intersection_identical()
{
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.0, 0.0, 1.0, 1.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), 1.0 );
}
#[test]
fn area_of_intersection_inside()
{
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.25, 0.25, 0.5, 0.5 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), 0.25 );
}
#[test]
fn area_of_intersection_outside()
{
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.5, -0.5, 1.0, 2.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), 0.5 );
}
#[test]
fn area_of_intersection_on_right()
{
  println!( "partial" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.5, 0.0, 1.0, 1.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), 0.5 );

  println!( "intersects on side" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 1.0, 0.0, 1.0, 1.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), 0.0 );
}
#[test]
fn area_of_intersection_on_left()
{
  println!( "partial" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), -0.5, 0.0, 1.0, 1.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), 0.5 );

  println!( "intersects on side" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), -1.0, 0.0, 1.0, 1.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), 0.0 );
}
#[test]
fn area_of_intersection_on_top()
{
  println!( "partial" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.0, 0.5, 1.0, 1.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), 0.5 );

  println!( "intersects on side" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.0, 1.0, 1.0, 1.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), 0.0 );
}
#[test]
fn area_of_intersection_on_bottom()
{
  println!( "partial" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.0, -0.5, 1.0, 1.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), 0.5 );

  println!( "intersects on side" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 0.0, -1.0, 1.0, 1.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), 0.0 );
}
#[test]
fn area_of_intersection_only_dot()
{
  println!( "right-bottom" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 1.0, -1.0, 1.0, 1.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), 0.0 );

  println!( "right-top" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 1.0, 1.0, 1.0, 1.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), 0.0 );

  println!( "left-bottom" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), -1.0, -1.0, 1.0, 1.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), 0.0 );

  println!( "left-top" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), -1.0, 1.0, 1.0, 1.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), 0.0 );
}
#[test]
fn area_of_intersection_no_rect()
{
  println!( "x is bigger" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 1.1, -1.0, 1.0, 1.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), -1.0 );

  println!( "y is bigger" );
  let rect1 = rect_make( String::from( "A" ), 0.0, 0.0, 1.0, 1.0 );
  let rect2 = rect_make( String::from( "B" ), 1.0, 1.1, 1.0, 1.0 );
  assert_eq!( rect1.intersection_area_get( &rect2 ), -1.0 );
}
