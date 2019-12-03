

#[derive(Default, Debug)]
pub struct Descriptor
{
    pub x : u32,
    pub y : u32,

    pub h: u32,
    pub w: u32,
}

pub struct FabricPiece {
  data: Vec<Vec<u32>>,
}

impl FabricPiece {

    // Create a new fabric piece.
    pub fn new() -> FabricPiece {
        FabricPiece{data: vec![vec![0; 1025]; 1025]}
    }

    // Populate the fabric with a single descriptor
    pub fn populate(&mut self, desc : &Descriptor) { 

        for dx in 0..desc.w {
            for dy in 0..desc.h {
                let x = dx + desc.x;
                let y = dy + desc.y;

                if x >= 1000 || y >= 1000 {
                    println!("x: {}, y: {}", x, y);
                }
                self.data[x as usize][y as usize] += 1;
            }
        }
    }

    // Find how many descriptors double booked a tile.
    pub fn double_booked_count(&self) -> u32 {

        let mut count = 0;

        for x in 0..1025 {
            for y in 0..1025 {
                if self.data[x][y] > 1 {
                    count += 1;
                }
            }
        }

        return count;
    }

    pub fn is_uncontended(&self, desc: &Descriptor) -> bool {

       for dx in 0..desc.w {
           for dy in 0..desc.h {
               let x = dx + desc.x;
               let y = dy + desc.y;

               if self.data[x as usize][y as usize] > 1 {
                   return false;
               }
           }
       }
  
       return true;
    }

    // Find how many tiles are in-use.
    #[cfg(test)]
    pub fn count(&self) -> u32 {

        let mut count = 0;

        for x in 0..1000 {
            for y in 0..1000 {
                if self.data[x][y] != 0 {
                    count += 1;
                }
            }
        }

        return count;
    }

    pub fn get(&self, x : u32, y: u32) -> u32 {
        self.data[x as usize][y as usize]
    }
}


#[test]
fn test_populate() {
    let mut fabric = FabricPiece::new();

    let desc = Descriptor {x:0, y:0, h:1000, w:1000};
    fabric.populate(&desc);

    assert_eq!(fabric.count(), 1000*1000);
}


/*
#[test]
fn test_boarder() {
    let mut fabric = FabricPiece::new();

    let desc1 = Descriptor {x:0, y:0, h:1000, w:1};
    fabric.populate(&desc1);

    let desc3 = Descriptor {x:1000, y:0, h:1000, w:1};
    fabric.populate(&desc3);

    let desc2 = Descriptor {x:0, y:0, h:1, w:1000};
    fabric.populate(&desc2);

    let desc4 = Descriptor {x:0, y:1000, h:1, w:1000};
    fabric.populate(&desc4);

    assert_eq!(fabric.count(), 1000*4);
}
*/

#[test]
fn test_populate_examples() {
    let mut fabric = FabricPiece::new();

    let desc1 = Descriptor {x:1, y:3, h:4, w:4};
    let desc2 = Descriptor {x:3, y:1, h:4, w:4};
    let desc3 = Descriptor {x:5, y:5, h:2, w:2};
    fabric.populate(&desc1);
    fabric.populate(&desc2);
    fabric.populate(&desc3);

    assert_eq!(fabric.double_booked_count(), 4);
}
