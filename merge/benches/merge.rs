use divan::Bencher;
use tw_merge::merge::tw_merge;

fn main() {
    divan::main();
}

const LENS: &[usize] = &[10, 20, 40, 60, 80];

#[divan::bench(
    args = LENS,
    max_time = 1,
    sample_count = 1000,
    sample_size = 1000
)]
fn tailwind_merge(bencher: Bencher, len: usize) {
    bencher
        .counter(divan::counter::CharsCount::new(len))
        .with_inputs(|| generate_random_classes(len))
        .input_counter(divan::counter::BytesCount::of_str)
        .bench_values(|class| tw_merge(class.as_str()));
}

// create a vec with the a length of len and fill it with random data
fn generate_random_classes(n: usize) -> String {
    let mut classes_vec = Vec::with_capacity(n);
    for _ in 0..n {
        let index = fastrand::usize(..TAILWIND_CLASSES.len());
        let element = TAILWIND_CLASSES[index];
        classes_vec.push(element);
    }
    classes_vec.join(" ")
}

const TAILWIND_CLASSES: [&str; 90] = [
    "bg-red-500",
    "text-xl",
    "p-4",
    "m-2",
    "rounded-lg",
    "shadow-md",
    "flex",
    "items-center",
    "justify-between",
    "gap-4",
    "hover:bg-blue-500",
    "active:text-white",
    "sm:text-base",
    "md:bg-green-500",
    "lg:p-8",
    "xl:m-10",
    "2xl:rounded-full",
    "focus:outline-none",
    "disabled:opacity-50",
    "group-hover:bg-purple-500",
    "transition-all",
    "duration-300",
    "ease-in-out",
    "transform",
    "rotate-45",
    "scale-105",
    "translate-x-2",
    "skew-y-6",
    "bg-[#123456]",
    "text-[14px]",
    "p-[30px]",
    "m-[5%]",
    "w-[calc(100%-2rem)]",
    "h-[50vh]",
    "gap-[2rem]",
    "grid-cols-[repeat(auto-fill,minmax(250px,1fr))]",
    "border-t-[3px]",
    "border-opacity-[0.85]",
    "shadow-[0_35px_60px_-15px_rgba(0,0,0,0.3)]",
    "backdrop-blur-[5px]",
    "bg-gradient-to-r",
    "from-[#6ee7b7]",
    "via-[#3b82f6]",
    "to-[#9333ea]",
    "text-[color:var(--custom-color)]",
    "hover:bg-[length:200px_100px]",
    "flex",
    "flex-col",
    "flex-row",
    "grid",
    "bg-blue-100",
    "bg-opacity-50",
    "bg-gradient-to-br",
    "from-teal-400",
    "to-blue-500",
    "p-4",
    "m-2",
    "space-x-4",
    "rounded-lg",
    "border-2",
    "border-dashed",
    "border-red-300",
    "shadow-xl",
    "opacity-75",
    "mix-blend-multiply",
    "w-1/2",
    "min-w-full",
    "max-w-xs",
    "h-12",
    "min-h-screen",
    "scale-90",
    "rotate-12",
    "translate-x-1/4",
    "skew-y-3",
    "origin-top-right",
    "cursor-pointer",
    "select-none",
    "resize-y",
    "transition-colors",
    "duration-200",
    "ease-in-out",
    "delay-150",
    "animate-spin",
    "transform-gpu",
    "hover:bg-pink-700",
    "active:text-white",
    "focus:ring-4",
    "focus:outline-none",
    "disabled:opacity-25",
    "visited:text-purple-600",
];
