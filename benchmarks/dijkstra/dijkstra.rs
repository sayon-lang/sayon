/*
Dijkstra: the shortest distances from one place to every other on a
pseudo-random map of n places, each with four one-way roads out of it, using
a binary heap written out as Sayon's is, rather than BinaryHeap, so that all
three versions do the same work.
*/

#[derive(Clone, Copy)]
struct Road {
    to: usize,
    cost: i64,
}

#[derive(Clone, Copy)]
struct Entry {
    distance: i64,
    place: usize,
}

/* Puts an entry in the heap, whose first `size` places hold its entries. */
fn put(heap: &mut Vec<Entry>, size: &mut usize, entry: Entry) {
    if *size == heap.len() {
        heap.push(entry);
    } else {
        heap[*size] = entry;
    }
    let mut child = *size;
    *size += 1;
    while child > 0 {
        let parent = (child - 1) / 2;
        if heap[parent].distance <= heap[child].distance {
            break;
        }
        heap.swap(parent, child);
        child = parent;
    }
}

/* Takes the entry with the least distance out of the heap. */
fn take(heap: &mut [Entry], size: &mut usize) -> Entry {
    let least = heap[0];
    *size -= 1;
    heap[0] = heap[*size];
    let mut parent = 0;
    loop {
        let left = 2 * parent + 1;
        if left >= *size {
            break;
        }
        let mut child = left;
        if left + 1 < *size && heap[left + 1].distance < heap[left].distance {
            child = left + 1;
        }
        if heap[parent].distance <= heap[child].distance {
            break;
        }
        heap.swap(parent, child);
        parent = child;
    }
    least
}

fn main() {
    let places: usize = std::env::args().nth(1).and_then(|a| a.parse().ok()).unwrap_or(1000);
    let mut roads: Vec<Vec<Road>> = Vec::new();
    for _ in 0..places {
        roads.push(Vec::new());
    }
    let mut state: i64 = 42;
    for from in 0..places {
        for _ in 0..4 {
            state = (state * 1103515245 + 12345) % 2147483648;
            let to = (state / 16) as usize % places;
            state = (state * 1103515245 + 12345) % 2147483648;
            let cost = state / 65536 % 100 + 1;
            roads[from].push(Road { to, cost });
        }
    }
    let mut distances: Vec<i64> = vec![-1; places];
    let mut heap: Vec<Entry> = Vec::new();
    let mut size = 0;
    put(&mut heap, &mut size, Entry { distance: 0, place: 0 });
    while size > 0 {
        let entry = take(&mut heap, &mut size);
        if distances[entry.place] >= 0 {
            continue;
        }
        distances[entry.place] = entry.distance;
        for road in &roads[entry.place] {
            if distances[road.to] < 0 {
                put(&mut heap, &mut size, Entry { distance: entry.distance + road.cost, place: road.to });
            }
        }
    }
    let (mut reached, mut total, mut farthest) = (0, 0, 0);
    for &distance in &distances {
        if distance >= 0 {
            reached += 1;
            total += distance;
            if distance > farthest {
                farthest = distance;
            }
        }
    }
    println!("{reached} places reached, distances add up to {total}, the farthest is {farthest}");
}
