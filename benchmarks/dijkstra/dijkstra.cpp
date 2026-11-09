/*
Dijkstra: the shortest distances from one place to every other on a
pseudo-random map of n places, each with four one-way roads out of it, using
a binary heap written out as Sayon's is, rather than std::priority_queue, so
that all three versions do the same work.
*/
#include <cstdio>
#include <cstdlib>
#include <utility>
#include <vector>

struct Road {
    long long to;
    long long cost;
};

struct Entry {
    long long distance;
    long long place;
};

/* Puts an entry in the heap, whose first `size` places hold its entries. */
static void put(std::vector<Entry>& heap, size_t& size, Entry entry) {
    if (size == heap.size()) {
        heap.push_back(entry);
    } else {
        heap[size] = entry;
    }
    size_t child = size;
    size += 1;
    while (child > 0) {
        size_t parent = (child - 1) / 2;
        if (heap[parent].distance <= heap[child].distance) break;
        std::swap(heap[parent], heap[child]);
        child = parent;
    }
}

/* Takes the entry with the least distance out of the heap. */
static Entry take(std::vector<Entry>& heap, size_t& size) {
    Entry least = heap[0];
    size -= 1;
    heap[0] = heap[size];
    size_t parent = 0;
    while (true) {
        size_t left = 2 * parent + 1;
        if (left >= size) break;
        size_t child = left;
        if (left + 1 < size && heap[left + 1].distance < heap[left].distance) child = left + 1;
        if (heap[parent].distance <= heap[child].distance) break;
        std::swap(heap[parent], heap[child]);
        parent = child;
    }
    return least;
}

int main(int argc, char** argv) {
    long long places = argc > 1 ? std::atoll(argv[1]) : 1000;
    std::vector<std::vector<Road>> roads(places);
    long long state = 42;
    for (long long from = 0; from < places; ++from) {
        for (int road = 0; road < 4; ++road) {
            state = (state * 1103515245 + 12345) % 2147483648LL;
            long long to = state / 16 % places;
            state = (state * 1103515245 + 12345) % 2147483648LL;
            long long cost = state / 65536 % 100 + 1;
            roads[from].push_back(Road{to, cost});
        }
    }
    std::vector<long long> distances(places, -1);
    std::vector<Entry> heap;
    size_t size = 0;
    put(heap, size, Entry{0, 0});
    while (size > 0) {
        Entry entry = take(heap, size);
        if (distances[entry.place] >= 0) continue;
        distances[entry.place] = entry.distance;
        for (const Road& road : roads[entry.place]) {
            if (distances[road.to] < 0) put(heap, size, Entry{entry.distance + road.cost, road.to});
        }
    }
    long long reached = 0, total = 0, farthest = 0;
    for (long long distance : distances) {
        if (distance >= 0) {
            reached += 1;
            total += distance;
            if (distance > farthest) farthest = distance;
        }
    }
    std::printf("%lld places reached, distances add up to %lld, the farthest is %lld\n", reached, total, farthest);
}
