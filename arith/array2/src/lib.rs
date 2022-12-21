use std::iter::Iterator;
//use std::iter::Chain;
//use std::slice::Iter;


pub struct Array2<T: Clone>{
	values: Vec<T>,
	width: usize,
	height: usize,
}

impl<T:Clone> Array2 <T> { 
    pub fn height(&self) ->usize{
        self.height
    }

    pub fn width(&self) ->usize{
        self.width
    }

    pub fn fill(width: usize, height: usize, val_type: T) -> Self{
        //this macro allows you to create a brand new vector with these parameetrs passed through
        let values = vec![val_type; width * height];
        //creating the Array2 structure
        Array2{
            width : width,
            height : height,
            values: values,
        }
    }

    pub fn from_row_major(width: usize, height:usize, values:Vec<T>)-> Self{
        let vec_size = width * height;
        if vec_size != values.len() {
            panic!(
                "The size of the vector did not match the number of elements currently in it"
            );

        }else{
        

            Array2 {
                width : width,
                height : height,
                values : values, 
            }
        }
    }  
    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        // The compiler knows to optimize away the div-mod ops.
        self.values
            .iter()
            .enumerate()
            .map(move |(i, v)| (i / self.width, i % self.width, v))
    }


	
    pub fn iter_col_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.width)
            .map(move |c| (c, self.values.iter().skip(c)))
            .flat_map(move |(c, col)| {
                col.step_by(self.width)
                    .enumerate()
                    .map(move |(r, val)| (r, c, val))
            })
    }

    pub fn get_element(&self, row_index:usize, column_index:usize) -> Option<&T> { //MAKE IT GENERALIZABLE AND POLYMORPHIC
        //option????? result????
        self.one_dim_idx(row_index,column_index).map(|index| &self.values[index])
    }


    pub fn get_mut_element(&mut self, row_index: usize, column_index: usize) -> Option<&mut T> {
        self.one_dim_idx(row_index, column_index).map(move |index| &mut self.values[index])
    }


    pub fn one_dim_idx(&self,row: usize, column: usize) -> Option<usize> {
        if row < self.height && column < self.width {
            Some(row * self.width + column)
        }else{
            None
        }
    }

    pub fn by_ref(&mut self)-> &mut Self
    where 
        Self:Sized,
    {
        self
    }

    pub fn elements_row_major(&self) -> &Vec<T> {
        &self.values
    }

}

#[cfg(test)]
mod tests{
    use crate::Array2;

    
    #[test]
    fn check2(){
        let vec = vec![0,1,2,3,4,5,6,7];
        let array = Array2::from_row_major(2,4,vec);
        
        let nothing2: Vec<usize> = array.iter_row_major().map(|(row_index,column_index,indexed_pixel)| {println!("{} {} {}",row_index, column_index, indexed_pixel); indexed_pixel * 2}).collect();
        
        println!("{:?}",nothing2);
        

    }
    
    #[test]
    fn check3(){
        let vec = vec![0,1,2,3,4,5,6,7];
        let array = Array2::from_row_major(2,4,vec);
        let nothing: Vec<usize> = array.iter_col_major().map(|(row_index,column_index,indexed_pixel)| {println!("{} {} {}",row_index, column_index, indexed_pixel); indexed_pixel * 2}).collect();
        println!("{:?}", nothing);
    }
    
    #[test]
    fn check4(){
        let vec = vec![0,1,2,3,4,5,6,7];
        let array = Array2::from_row_major(2,4,vec);
        let element_check = array.get_element(0,1);
        match element_check{
            Some(x) => println!("Result: {}",x),
            None => println!("Coordinate does not exist for array"),
        }
    }
}

