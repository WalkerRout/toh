/*
- shift n-1 disks from A to B using C
- shift last disk from A to C
- shift n-1 disks from B to C using A
*/

fn toh(n: i32, src: char, aux: char, dest: char) {
  match n {
    0 => return,
    _ => {
      println!("n is: {}", n);
      toh(n - 1, src, dest, aux); // from a to b using c
      println!("Disk {} moved from {} to {}", n, src, dest);
      toh(n - 1, aux, src, dest); // from b to c using a
    }
  }
}

fn main() {
  toh(3, 'A', 'B', 'C');
}
