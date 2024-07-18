# Parallelized-Image-Processing-Program-funpar-
 Develop an image processing application that applies various filters (e.g., blur, sharpen, changing brightness(light and dark)) using concurrent programming techniques in Rust to significantly speed up the processing time compared to a single-threaded approach.

**How to use (step  by step tutorial):**

1. Please, go to the main file where every files with different functions will all be run there. //main file is located in /project/src/main.rs
2. Put in your picture path in the variable 's' so that the program can access and begin to add filters.
3. Click the run button and run the main file if you are using an editor or compile and run if you are using terminal.
4. It may take a moment to process every filters in one go, but all the modified picture should be outside the folder 'src'.
5. If you want to intensify or change the level in each filters, you can also do so by changing the value of 'blur_radius' for blur, 'brightening_factor'for brighten, and 'darkening_factor' for darken.

**Evaluation and Results:**
1. Performance: **Blur Function**
Algorithm:
The blur function involves iterating over each pixel and considering its neighboring pixels within a specified blur_radius.
For each pixel, the nested loops over the blur radius create an additional computation.

Runtime Analysis:
Outer loop over all pixels: O(w * h)
Inner loops over the blur radius: O((2 * blur_radius + 1)^2) for each pixel.
Total time complexity: O(w * h * (2 * blur_radius + 1)^2)
Using parallel processing with Rayon, the workload is distributed across multiple threads, which can significantly reduce the actual runtime.
______________________________________________________________________________________________________________________________________________
2. Performance: **Sharpen Function**
Algorithm:
The sharpen function uses a convolution kernel, typically a 3x3 matrix, to process each pixel.
For each pixel, the function applies the kernel to the neighboring pixels.

Runtime Analysis:
Outer loop over all pixels: O(w * h)
Inner loop over the 3x3 kernel: O(3 * 3) for each pixel.
Total time complexity: O(w * h * 3 * 3) = O(9 * w * h) = O(w * h)
Parallel processing with Rayon helps distribute the workload, reducing the actual runtime.

______________________________________________________________________________________________________________________________________________
3. Performance: **Darken Function**
Algorithm:
The darken function multiplies each pixel's RGB values by a darkening factor.
Runtime Analysis:

Single loop over all pixels: O(w * h)
Each pixel's RGB value modification: O(1)
Total time complexity: O(w * h)
Parallel processing efficiently handles the pixel-wise operations and runtime should be O(w*h) but got divided into each threads.
______________________________________________________________________________________________________________________________________________
4. Performance: **Brighten Function**
Algorithm:
The brighten function multiplies each pixel's RGB values by a brightening factor.
Runtime Analysis:

Single loop over all pixels: O(w * h)
Each pixel's RGB value modification: O(1)
Total time complexity: O(w * h)
Parallel processing ensures efficient handling of the pixel-wise operations.
______________________________________________________________________________________________________________________________________________



