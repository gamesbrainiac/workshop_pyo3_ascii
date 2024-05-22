import sys
from PIL import Image
from pictoascii import convert_img_to_ascii


class ASCIIArtGenerator:
    def __init__(self, ascii_chars="@%#*+=-:. "):
        # Initialize the generator with a string of ASCII characters.
        # The characters should range from darkest to lightest to visually represent different shades.
        self.ascii_chars = ascii_chars

    def scale_image(self, image, new_width=100):
        # Calculate the new dimensions of the image maintaining the aspect ratio.
        width, height = image.size
        aspect_ratio = height / width
        new_height = int(new_width * aspect_ratio)
        
        # Resize the image to the new dimensions.
        scaled_image = image.resize((new_width, new_height))
        return scaled_image

    def convert_to_grayscale(self, image):
        # Convert the image to grayscale to simplify processing.
        # This discards any color information in the image.
        return image.convert('L')

    def map_pixels_to_ascii(self, image):
        # Get all pixels from the grayscale image.
        pixels = image.getdata()
        # Calculate the number of available ASCII characters.
        ascii_len = len(self.ascii_chars)
        # Convert each pixel to an ASCII character by mapping the pixel value (0-255)
        # to the corresponding index in the ascii_chars string.
        ascii_str = ''.join(
            self.ascii_chars[min(pixel * ascii_len // 256, ascii_len - 1)]
            for pixel in pixels
        )
        return ascii_str

    def generate_ascii_art(self, image_path, width=100):
        # Open the image from the given path.
        with Image.open(image_path) as image:
            # Scale and convert the image to grayscale.
            image = self.scale_image(image, width)
            image = self.convert_to_grayscale(image)

            # Map each pixel of the grayscale image to an ASCII character.
            ascii_str = self.map_pixels_to_ascii(image)
            # The total number of characters in the ASCII string.
            pixel_count = len(ascii_str)
            # Group the ASCII characters into lines to form the final ASCII art.
            ascii_art = "\n".join(
                ascii_str[i:(i + width)] for i in range(0, pixel_count, width)
            )

            return ascii_art


if __name__ == "__main__":
    generator = ASCIIArtGenerator()
    image_path = sys.argv[1] if len(sys.argv) > 1 else "./image.jpg"
    with open("python_coffee.txt", "w+") as f:
        f.write(generator.generate_ascii_art(image_path, 100))
    
    with open("rust_coffee.txt", "w+") as f:
        f.write(convert_img_to_ascii(image_path, 100))

