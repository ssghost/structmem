#include <initializer_list>
#include <iostream>
#include <memory>
#include <vector>

struct Seq {
    int id;
    std::shared_ptr<std::vector<int>> vals;
};

__attribute__((noinline))
auto upped_id(Seq seq) -> Seq {
    seq.id += 1;
    return seq;
}

__attribute__((noinline))
auto report(const Seq& seq) -> void {
    std::cout << seq.id << " has " << seq.vals->size() << "\n";
}

__attribute__((noinline))
auto report_changes(Seq a) -> void {
    auto b = upped_id(a);
    a.vals->push_back(0);
    report(a);
    report(b);
}

auto main() -> int {
    auto a = Seq {
        .id = 5,
        .vals = std::make_shared<std::vector<int>>(
            std::initializer_list { 6, 7 }
        ),
    };
    report_changes(a);
}
