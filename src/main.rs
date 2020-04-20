/**
*  Get a diff of a pixel and all its surroudning pixels 9x9(beat andrew).  Recursively track each
* pixel from the bottom up.  when doing this check if it needs to be done based on a top pixel removal.  


*/

extern crate image;

use std::time::Instant;
use std::env;
use std::cmp;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage, DynamicImage, Pixel};

struct DPValue<'a>{
  diff_total: u64,
  pix: &'a image::Rgb<u8>,
  wasProcessed: bool
}


fn getImage(image_name: &std::string::String) -> RgbImage {

  //Get Image from /images
  let src_dir = env::current_dir().unwrap();
  let path_to_image = format!("{}/../images/{}", src_dir.display(), image_name);
  
  let image = image::open(path_to_image).unwrap();
  
  image.into_rgb()
}

fn SaveImage<'x>(values: &mut Vec<Vec<DPValue<'x>>>) {

  let xLen: u32 = values.len() as u32;
  let yLen: u32 = values[0].len() as u32;


  let mut img = RgbImage::new(yLen, xLen);

  for x in 0..(values.len() as u32) {
    for y in 0..(values[x as usize].len() as u32) {
      let xVal = x as usize;
      let yVal = y as usize;
      img.put_pixel(y,x, *values[xVal][yVal].pix);
    }
  }

  println!("about to crsah3?");
  //let image_buf = from_vec(values.len(), values[0].len(), img);

  img.save("test.png").unwrap();
}

//Function 
fn getTotalPixelDiff<'x>(h: usize, w: usize,values: &mut Vec<Vec<DPValue<'x>>>) -> u64 {
  
  // Get value of all pixels in 9x9 grid and add them then average that
  // Get GetTotalPixelDiff of topleft,top,topright pixels
  // add this value to those diffs; return. 


  //note when doing through check against lastTopRemovedPixel(later defined) to see
  // if this value needs to be reevaluated

  let mut personalDiff: u64 = 0;
  let mut sum:          u32 = 0;
  let mut divisor:      u32 = 0;
  let width:            i32 = w as i32;
  let height:           i32 = h as i32;

  let mut sumAboveDiffs:u64 = 0;
  
  if (values[h][w].wasProcessed) {
    //println!("Auto Return X: {} Y: {} diff : {}", w, h, values[w][h].diff_total);
    return values[h][w].diff_total
  }
  
  let selfR = values[h][w].pix[0] as i32;
  let selfG = values[h][w].pix[1] as i32;
  let selfB = values[h][w].pix[2] as i32;

  for y in cmp::max(0,height-1)..cmp::min((values.len()) as i32, height+2) {
    for x in cmp::max(0,width-1)..cmp::min((values[0].len()) as i32, width+2) {

      let xVal = x as usize;
      let yVal = y as usize;
      let mut rgbSum: i32 = 0;

      rgbSum += ((values[yVal][xVal].pix[0] as i32) - selfR).abs();
      rgbSum += ((values[yVal][xVal].pix[1] as i32) - selfG).abs();
      rgbSum += ((values[yVal][xVal].pix[2] as i32) - selfB).abs();

      sum += (rgbSum as u32) / 3;
      divisor += 1;
    }
  }
  
  //println!("What is the personal Diff before dividing {}", sum);
  personalDiff = (sum/(divisor - 1)).into();
  //println!("What is the personal Diff after dividing {}", personalDiff);
  //println!("What is the personal divisor after divisor {}", divisor);

  if h == 0 {
    values[h][w].diff_total = personalDiff;
    values[h][w].wasProcessed = true;
    return values[h][w].diff_total
  }

  let mut bottom: u64 = 0;
  for y in cmp::max(0,width-1)..cmp::min((values[0].len()) as i32,width+2) {
    let yVal = y as usize;
    sumAboveDiffs += getTotalPixelDiff(h-1, yVal, values);
    bottom += 1;
  }

  if (bottom > 0)
  {
    values[h][w].diff_total = (sumAboveDiffs + personalDiff) / bottom;
  }
  else
  {
    values[h][w].diff_total = (sumAboveDiffs + personalDiff);
  }

  values[h][w].wasProcessed = true;

  values[h][w].diff_total
}

//remove pixel, shift everything pop the end
fn removePixel<'x> (h: usize, w: usize,values: &mut Vec<Vec<DPValue<'x>>>) -> usize{

  //println!("RemovingPixel {},{}, diff_Value{}?", h, w, values[h][w].diff_total);
  values[h].remove(w);
  if h == 0 {
    return w
  }
  
  let mut minIndex: usize = w;
  let width:            i32 = w as i32;
  let height:           i32 = h as i32;

  for y in cmp::max(0,width-1)..cmp::min(width+2,values[0].len() as i32) {
    let yVal = y as usize;
    if values[h-1][yVal].diff_total < values[h-1][minIndex].diff_total {
      minIndex = yVal;
    }
  }

  removePixel(h-1,minIndex, values)
}

fn main() {

  let args: Vec<String> = env::args().collect();
  let mut file_name = &"seattleSkyline.jpg".to_owned();
  if args.len() > 1
  {
    file_name = &args[1];
  }

  let image = getImage(file_name);

  let (width, height) = image.dimensions();
  let mut width = width as usize;
  let height = height as usize;
  let mut diffVec = Vec::new();
  
  println!("About to create vector");
  for x in 0..height {
    let mut internalVec = Vec::new();
    for y in 0..width {
      let mut val: DPValue = DPValue {
        pix: image.get_pixel(y as u32,x as u32),
        diff_total: 0,
        wasProcessed: false
      };
      internalVec.push(val);
    }
    diffVec.push(internalVec);
  }
  //println!("Createdvector");
  //End init
  let mut start = Instant::now();
  for count in 0..100
  {
    println!("Function took {:?}", start.elapsed());
    start = Instant::now();

    for y in 0..width {
      diffVec[height-1][y].diff_total = getTotalPixelDiff(height-1, y, &mut diffVec);
    } 

    let mut minIndex: usize = 0;
    for y in 1..width {
      let yVal = y as usize;
      //println!("DiffTtoal of y: {} = {}", yVal, diffVec[height-1][yVal].diff_total); 

      if diffVec[height-1][yVal].diff_total < diffVec[height-1][minIndex].diff_total {
        minIndex = yVal;
      }
    }

    println!("Removing pixels start {}", minIndex);
    let topW = removePixel(height-1,minIndex, &mut diffVec) as i32;
    println!("Finished removingPixels {}", topW);

    //println!("Diff vel len {}", diffVec.len());
    for x in 0..diffVec.len(){
      for y in 0..diffVec[x].len(){
        if ((y as i32) > topW - (x as i32) && (y as i32) < topW + (x as i32)) {
          diffVec[x as usize][y as usize].wasProcessed = false;  
        }
      }
    }
    width -= 1;
    println!("Iteration {}", count);
  }

  SaveImage(&mut diffVec);
  
  /*for pixel in image.pixels() {
    let rgb_pixel = pixel.to_rgb();
    println!("{:?}", rgb_pixel);
    println!("{:?}", rgb_pixel[0]);
    // Loop through all pixels at the bottom calling getTotalPixelDiff on each
    // once your through that loop through the bottom and look for min value. Then from 
    // there check every pixel above and repeat removing along the way.

  }
  */
}

