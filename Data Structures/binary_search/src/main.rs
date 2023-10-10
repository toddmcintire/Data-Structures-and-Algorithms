fn main() {

    let my_vec = vec!(50, 52, 54, 57, 60, 70);

    binary_search(my_vec, 57);
}

fn binary_search(list: Vec<i32>, item: i32) -> i32{
   let mut low = 0;
   let mut high = list.len() - 1;

   while low <= high {
       let mid = low + high;
       let guess = list[mid];
       println!("{}",guess);

       if guess == item {
            return mid.try_into().unwrap()
       } else if guess > item {
            high = mid -1;
            println!("too high");
       } else {
            low = mid + 1;
            println!("too low");
       }
   }

   println!("not in vector");
   return 0;
}
