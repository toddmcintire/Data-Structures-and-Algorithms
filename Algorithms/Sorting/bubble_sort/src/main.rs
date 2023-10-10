//Todd McIntire 6/4/2023 9:22pm
//bubble sort algorithm with explanation
//bubble sort works by comparing two elements, if the left element is bigger then the second then we swap them and continue down the line

fn main() {
    let my_vec = vec!(2, 8, 5, 3, 9, 4, 1);
    println!("{:?}", bubble_sort(my_vec));
}

fn bubble_sort(mut input: Vec<i32>) -> Vec<i32>{
    let mut swapped: bool;
    let mut temp: i32;

    //loop from start to end of elements, this loop facilitates the loop that does the sorting
    for _outer in 0..input.len() - 1 {

        //on each of our _outer loops set/reset swapped to false
        swapped = false;

        //loop from start to end of elements, this loop will check for the elements
        //the inner loop runs from start to finish each iteration of the previous _outer loop so that every element is covered
        for inner in 0..input.len() - 1 {
            //if input at current inner loop is greater than input at current inner loop + 1 then do code below
            if input[inner] > input[inner + 1] {
                //swap input[inner] and input[inner + 1]

                //place input at inner into temp
                temp = input[inner];
                //replace input at inner with input at inner + 1
                input[inner] = input[ inner + 1];
                //replace input at inner + 1 with temp value
                input[inner + 1] = temp;
                //change swapped to true
                swapped = true;

            }
        }

        // if no two elements were swapped by inner loop then break

        //if after running the inner loop swapped is not changed to true, then we no two elements were swapped then all must be in order and we can prematurely break out of the loop
        if swapped == false {
            break;
        }
    }

    //return input
    input
}
