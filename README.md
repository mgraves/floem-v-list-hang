

### BUG DEMO REPO for `floem` UI framework `virtual_list()` problem.

#### This may be my error, if so, please let me know at `groupmg@gmail.com.`

The code in this repo is stripped down as far as I could get it to show a `virtual_lsit()` getting in am infinite loop, 
and locking up the app. This seems to be a recent change, as the slightly more
complex text app it came from was working fine with the same implementation a week ago.

(OR, this was something I goofed up?!)


![screenshot](./img.png)

This app depends on the current version of `floem` at https://github.com/lapce/floem` as of 3 Jan, 2024.
This code has only been tested on MacOS at this point by me.

To reproduce the lockup:

1. Build and launch the app
2. Click the 'A' Button,
3. A list entry with the label "A" should appear in the upper left of the window.
4. Click the "A" a second time.
5. The button will not exit it's down/highlight state, and the app locks up.

There is a `backtrace` available at [./backtrace.txt](./backtrace.txt). Looking at that,
this appears to tbe problematic repeating stretch:

```
    frame #10: 0x000000010a48481c floem-v-list-hang`<taffy::compute::GenericAlgorithm as taffy::compute::LayoutAlgorithm>::measure_size(tree=0x00007fe54000caa8, node=slotmap::DefaultKey @ 0x00007ff7b63f0664, known_dimensions=<unavailable>, parent_size=<unavailable>, available_space=<unavailable>, sizing_mode=ContentSize) at mod.rs:113:9
    frame #11: 0x000000010a490799 floem-v-list-hang`taffy::compute::flexbox::determine_flex_base_size(tree=0x00007fe54000caa8, constants=0x00007ff7b63f0cd0, available_space=taffy::geometry::Size<taffy::style::dimension::AvailableSpace> @ 0x00007ff7b63f0f10, flex_items=size=1) at flexbox.rs:724:13
    frame #12: 0x000000010a48dc75 floem-v-list-hang`taffy::compute::flexbox::compute_preliminary(tree=0x00007fe54000caa8, node=slotmap::DefaultKey @ 0x00007ff7b63f0eb8, known_dimensions=taffy::geometry::Size<core::option::Option<f32>> @ 0x00007ff7b63f1250, parent_size=taffy::geometry::Size<core::option::Option<f32>> @ 0x00007ff7b63f1260, available_space=taffy::geometry::Size<taffy::style::dimension::AvailableSpace> @ 0x00007ff7b63f1270, run_mode=PeformLayout) at flexbox.rs:252:5
    frame #13: 0x000000010a48d977 floem-v-list-hang`taffy::compute::flexbox::compute(tree=0x00007fe54000caa8, node=slotmap::DefaultKey @ 0x00007ff7b63f11a0, known_dimensions=taffy::geometry::Size<core::option::Option<f32>> @ 0x00007ff7b63f12e0, parent_size=taffy::geometry::Size<core::option::Option<f32>> @ 0x00007ff7b63f12f0, available_space=taffy::geometry::Size<taffy::style::dimension::AvailableSpace> @ 0x00007ff7b63f1300, run_mode=PeformLayout, sizing_mode=InherentSize) at flexbox.rs:218:5
    frame #14: 0x000000010a48d4be floem-v-list-hang`<taffy::compute::flexbox::FlexboxAlgorithm as taffy::compute::LayoutAlgorithm>::perform_layout(tree=0x00007fe54000caa8, node=slotmap::DefaultKey @ 0x00007ff7b63f12d4, known_dimensions=<unavailable>, parent_size=<unavailable>, available_space=<unavailable>, sizing_mode=InherentSize) at flexbox.rs:37:9
    frame #15: 0x000000010a484dde floem-v-list-hang`taffy::compute::compute_node_layout [inlined] taffy::compute::compute_node_layout::perform_computations(tree=0x00007fe54000caa8, node=slotmap::DefaultKey @ 0x00007ff7b63f171c, known_dimensions=taffy::geometry::Size<core::option::Option<f32>> @ 0x00007ff7b63f14a0, parent_size=taffy::geometry::Size<core::option::Option<f32>> @ 0x00007ff7b63f14b0, available_space=taffy::geometry::Size<taffy::style::dimension::AvailableSpace> @ 0x00007ff7b63f14c0, run_mode=PeformLayout, sizing_mode=InherentSize) at mod.rs:174:17
    frame #16: 0x000000010a484d45 floem-v-list-hang`taffy::compute::compute_node_layout(tree=0x00007fe54000caa8, node=slotmap::DefaultKey @ 0x00007ff7b63f1470, known_dimensions=taffy::geometry::Size<core::option::Option<f32>> @ 0x00007ff7b63f17f0, parent_size=taffy::geometry::Size<core::option::Option<f32>> @ 0x00007ff7b63f1800, available_space=taffy::geometry::Size<taffy::style::dimension::AvailableSpace> @ 0x00007ff7b63f1810, run_mode=PeformLayout, sizing_mode=InherentSize) at mod.rs:196:34
```
... or some close variant of that sequence.







