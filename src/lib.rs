/*!
```
use mkbs::MKBS;

let keys = &[123, 3131];
let vec_of_vals = (0..4123).collect::<Vec<_>>();

let results = vec_of_vals.mkbs(keys);
println!("{results:?}");
// [Ok(123), Ok(3131)]
```
If the value is not found then [`Result::Err`] is returned, containing
the index where a matching element could be inserted while maintaining
sorted order, just like in the std library binary search.
In the case of [`MKBS::mkbs()`], it mostly returns [`Option::None`] in the place of possible indices.
If you want every not-found element to have a possible index then you should use [`MKBS::mkbs_all()`], note that it is almost two times slower.
 */

type SearchResult = Result<usize, Option<usize>>;

#[inline]
fn get_middle(left: usize, right: usize) -> usize {
    (left) + (((right) - (left)) >> 1)
}

fn _mkbs_all_by<T: Ord>(
    arr: &[T],
    arr_l: usize,
    arr_r: usize,
    keys: &[T],
    keys_l: usize,
    keys_r: isize,
    results: &mut [SearchResult],
) {
    if keys_r < keys_l as isize {
        return;
    }

    let keys_middle = get_middle(keys_l, keys_r as usize);

    let pos = arr[arr_l..=arr_r].binary_search(&keys[keys_middle]);

    let pos = if let Ok(i) = pos {
        Ok(i + arr_l)
    } else if let Err(i) = pos {
        Err(Some(i + arr_l))
    } else {
        unreachable!()
    };

    results[keys_middle] = pos;

    let found = pos.is_ok();
    let pos = pos.unwrap_or_else(|x| x.unwrap());

    _mkbs_all_by(
        arr,
        arr_l,
        pos.saturating_sub(1),
        keys,
        keys_l,
        keys_middle as isize - 1,
        results,
    );

    _mkbs_all_by(
        arr,
        if found { pos + 1 } else { pos },
        arr_r,
        keys,
        keys_middle + 1,
        keys_r,
        results,
    );
}

fn _mkbs_by<T: Ord>(
    arr: &[T],
    arr_l: usize,
    arr_r: usize,
    keys: &[T],
    keys_l: usize,
    keys_r: isize,
    results: &mut [Result<usize, Option<usize>>],
) {
    if keys_r < keys_l as isize {
        return;
    }

    let keys_middle = get_middle(keys_l, keys_r as usize);

    if keys[keys_middle] < arr[arr_l] {
        _mkbs_by(arr, arr_l, arr_r, keys, keys_middle + 1, keys_r, results);
        return;
    }

    if keys[keys_middle] > arr[arr_r] {
        _mkbs_by(
            arr,
            arr_l,
            arr_r,
            keys,
            keys_l,
            keys_middle as isize - 1,
            results,
        );
        return;
    }

    let pos = arr[arr_l..=arr_r].binary_search(&keys[keys_middle]);

    let pos = if let Ok(i) = pos {
        Ok(i + arr_l)
    } else if let Err(i) = pos {
        Err(Some(i + arr_l))
    } else {
        unreachable!()
    };

    results[keys_middle] = pos;
    let found = pos.is_ok();
    let pos = pos.unwrap_or_else(|x| x.unwrap());

    _mkbs_by(
        arr,
        arr_l,
        pos - 1,
        keys,
        keys_l,
        keys_middle as isize - 1,
        results,
    );
    _mkbs_by(
        arr,
        if found { pos + 1 } else { pos },
        arr_r,
        keys,
        keys_middle + 1,
        keys_r,
        results,
    );
}

pub trait MKBS<T, const N: usize>
where
    T: Ord,
{
    fn mkbs_all(&self, keys: &[T; N]) -> [SearchResult; N];
    fn mkbs(&self, keys: &[T; N]) -> [SearchResult; N];
}

impl<T, const N: usize> MKBS<T, N> for [T]
where
    T: Ord,
{
    fn mkbs_all(&self, keys: &[T; N]) -> [SearchResult; N] {
        debug_assert_ne!(N, 0);
        let mut res = [Err(None); N];
        _mkbs_all_by(
            self,
            0,
            self.len() - 1,
            keys,
            0,
            (keys.len() - 1) as isize,
            &mut res,
        );
        res
    }

    fn mkbs(&self, keys: &[T; N]) -> [SearchResult; N] {
        debug_assert_ne!(N, 0);
        let mut res = [Err(None); N];
        _mkbs_by(
            self,
            0,
            self.len() - 1,
            keys,
            0,
            keys.len() as isize - 1,
            &mut res,
        );
        res
    }
}

impl<T, const N: usize> MKBS<T, N> for Vec<T>
where
    T: Ord,
{
    fn mkbs_all(&self, keys: &[T; N]) -> [SearchResult; N] {
        self.as_slice().mkbs_all(keys)
    }

    fn mkbs(&self, keys: &[T; N]) -> [SearchResult; N] {
        self.as_slice().mkbs(keys)
    }
}

#[cfg(test)]
mod tests {
    use crate::{SearchResult, MKBS};
    const TEST_AMOUNT: usize = 40;
    const ARR_DUP_SIZE: usize = 3_000_000;
    const KEYS_DUP_SIZE: usize = 3_000_000;
    const KEYS_SIZE: usize = 2000;

    #[test]
    fn test_both() {
        test_mkbs(MKBS::mkbs_all_by, test_results_all);
        test_mkbs(MKBS::mkbs_by, test_results);
    }

    fn test_mkbs<F, A>(mkbs_func: F, asserter: A)
    where
        F: Fn(&[i32], &[i32; KEYS_SIZE]) -> [SearchResult; KEYS_SIZE],
        A: Fn(&[SearchResult], &[i32], &[i32]),
    {
        use rand::Rng;

        let mut total = 0.0;
        let mut rng = rand::thread_rng();

        for test_i in 1..=TEST_AMOUNT {
            let mut arr = vec![0i32; ARR_DUP_SIZE];
            let mut keys = vec![0i32; KEYS_DUP_SIZE];

            println!("TEST #{test_i}");
            arr.fill_with(|| rng.gen());
            arr.sort_unstable();
            arr.dedup();

            keys.fill_with(|| rng.gen());
            keys.sort_unstable();
            keys.dedup();

            let keys_ = unsafe { &*(keys.as_slice() as *const [i32] as *const [i32; KEYS_SIZE]) };
            println!("arr size: {}, keys size: {}", arr.len(), keys_.len());
            let tic = std::time::Instant::now();
            let results = mkbs_func(arr.as_slice(), keys_);
            let elapsed = tic.elapsed().as_secs_f32();

            total += elapsed;
            println!("elapsed time: {elapsed}");
            asserter(&results, &keys, &arr);
        }

        println!("avg: {}", total / TEST_AMOUNT as f32);
    }

    fn test_results(results: &[SearchResult], keys: &[i32], arr: &[i32]) {
        let mut nos = 0;
        let mut yes = 0;
        for (keys_i, r) in results.iter().enumerate() {
            match r {
                Ok(i) => {
                    assert_eq!(arr[*i], keys[keys_i]);
                }
                Err(Some(i)) => {
                    assert!(i > &arr.len() || arr[*i] != keys[keys_i]);
                    yes += 1;
                }
                Err(None) => nos += 1,
            }
        }
        println!("all passed, suggestions: {yes} nosuggestions: {nos}");
    }

    fn test_results_all(results: &[SearchResult], keys: &[i32], arr: &[i32]) {
        let mut nos = 0;
        let mut yes = 0;
        for (keys_i, r) in results.iter().enumerate() {
            match r {
                Ok(i) => {
                    assert_eq!(arr[*i], keys[keys_i]);
                }
                Err(Some(i)) => {
                    if *i < arr.len() {
                        assert_ne!(arr[*i], keys[keys_i]);
                    } else if *i == 0 {
                        assert!(keys[keys_i] < arr[*i])
                    } else if *i == arr.len() {
                        assert!(keys[keys_i] > arr[arr.len() - 1])
                    } else {
                        assert!(keys[keys_i] > arr[*i - 1] && keys[keys_i] > arr[*i + 1])
                    }
                    yes += 1;
                }
                Err(None) => nos += 1,
            }
        }
        println!("all passed, suggestions: {yes} nosuggestions: {nos}");
        assert_eq!(nos, 0);
    }
}
