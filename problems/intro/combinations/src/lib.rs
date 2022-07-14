#![forbid(unsafe_code)]

// Algo was borrowed from https://forum.sources.ru/index.php?showtopic=330364&st=0&
pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    match (arr.len(), k) {
        (_, 0) => {
            return vec![vec![]];
        }
        (0, _) => {
            return Vec::<Vec<i32>>::new();
        }
        _ => (),
    }
    let mut res: Vec<Vec<i32>> = vec![];
    let mut run = (0..k as i32).collect::<Vec<i32>>();
    let mut s: Vec<i32>;
    let mut p: i32 = (k as i32) - 1;
    let n: i32 = (arr.len() as i32) - 1;
    let new_k = k - 1;
    // println!("init p {}", p);
    while p >= 0 {
        // println!(" run before {:?}", run);
        let inner_run: Vec<_> = run.iter().map(|&x| arr[x as usize]).collect::<Vec<_>>();
        res.push(inner_run);
        if run[new_k] == n {
            p -= 1;
            // println!("run[{}] : {} == n : {} => p = {}", new_k, run[new_k], n, p);
        } else {
            p = new_k as i32;
            // println!("run[{}]: {} != n : {} => p = {}", new_k, run[new_k], n, p);
        }
        // println!("some {} {}", p, new_k);
        if p >= 0 {
            let run_right = (p..=new_k as i32)
                .map(|x| {
                    println!("map run[{}]: {} {}", p, run[p as usize], x);
                    let y: i32 = run[(p as usize)] + x - (p as i32) + 1;
                    y
                })
                .collect::<Vec<i32>>();
            let run_left = run[0..run.len() - run_right.len()]
                .iter()
                .copied()
                .collect::<Vec<i32>>();
            s = run_left
                .iter()
                .chain(run_right.iter())
                .copied()
                .collect::<Vec<i32>>();
            run = s;
        }
    }
    res
}

fn some() {
    let a: Vec<i32> = vec![0, 1, 2];
    let b: Vec<i32> = vec![3, 4];
    let c = a[0..=a.len() - b.len()]
        .iter()
        .copied()
        .collect::<Vec<i32>>();
    let d = c.iter().chain(b.iter()).copied().collect::<Vec<i32>>();
    println!("some() d {:?}", d);
}

pub fn main() {
    let _s = combinations(&[4, 5, 3, 2, 1], 3);
    // some();
}

//     for i := 1 to k do
//       A[i] := i; //Первое подмножество
//     p := k;
//     while p >= 1 do
//     begin
//       writeln(A[1],..., A[k]); //вывод очередного сочетания
//       if A[k] = n then
//         p := p - 1
//       else
//         p := k;
//       if p >= 1 then
//         for i := k downto p do
//           A[i] := A[p] + i - p + 1;
//     end;
//   end;
// let mut sorted_arr = arr.iter().copied().collect::<Vec<i32>>();
// sorted_arr.sort();
// println!("Orig arr {:?} \t sorted {:?}", arr, sorted_arr);
// let run = sorted_arr[..k].iter().copied().collect::<Vec<i32>>();
// for i in (0..run.len()).rev() {
//     let mut i_run = run.iter().copied().collect::<Vec<_>>();
//     for ite in i..sorted_arr.len() {
//         i_run[i] = sorted_arr[ite];
//         println!("vec of slices 1 {:?}", i_run);
//     }
//     for it in (i..run.len()).rev() {
//         let mut it_run = i_run.iter().copied().collect::<Vec<_>>();
//         for ite in it..sorted_arr.len() {
//             it_run[it] = sorted_arr[ite];
//             println!("vec of slices 1 {:?}", it_run);
//         }
//         println!("vec of slices 2 {} {}", i, it,);
//     }
// }
// for (i, item) in sorted_arr.iter().enumerate() {
//     println!("iterating {} {}", i, item);
//     let mut run = sorted_arr
//         .iter()
//         .cycle()
//         .skip(i)
//         .take(k)
//         .copied()
//         .collect::<Vec<i32>>();
//     run.sort();
//     println!("run before {:?}", run);
//     for i in (0..run.len()).rev() {
//         for it in (i..run.len()).rev() {
//             let mut it_run = run.iter().copied().collect::<Vec<_>>();
//             for ite in i..sorted_arr.len() {
//                 it_run[it] = sorted_arr[ite];
//                 println!("vec of slices 1 {:?}", it_run);
//             }
//         }

//         // for ite in 0..run.len() {
//         //     if ite == it {
//         //         continue;
//         //     }
//         //     less_one_elem.push(run[ite]);
//         // }
//         // println!("vec of slices 1 {:?}", less_one_elem);
//         // for
//         //     for itera in (0..=less_one_elem.len()).rev() {
//         //         // non inclusive
//         //         let mut v_less_one_element: Vec<i32> =
//         //             less_one_elem.iter().copied().collect::<Vec<_>>();
//         //         v_less_one_element.insert(itera, run[it]);
//         //         println!("vec of slices 2 {:?}", v_less_one_element);
//         //         res.push(v_less_one_element);
//         //     }
//         // }
//         // let i = run.len() - 1;
//         // for it in (0..i).rev() {
//         //     // non inclusive
//         //     let mut v_less_one_element: Vec<i32> = run.iter().copied().collect::<Vec<_>>();
//         //     v_less_one_element.insert(it, run[i]);
//         //     println!("vec of slices {:?}", v_less_one_element);
//         //     res.push(v_less_one_element);
//     }
// }
// res = vec![sorted_arr.iter().copied().collect::<Vec<i32>>(); 1];
// println!("vec of slices {:?}", res);
// let mut s: Vec<i32> = vec![2, 3, 4, 5];
// // s.iter().rev().for_each(|x| println!("iter val {}", x));
// let i = s.len() - 1;
// for it in (0..i).rev() { // non inclusive
//     let mut v_less_one_element: Vec<i32>  = s[..i].iter().copied().collect::<Vec<i32>>();
//     v_less_one_element.insert(it, s[i]);
//     println!("vec of slices {:?}", v_less_one_element);
// }

// res
