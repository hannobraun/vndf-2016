#include "images.h"


image  loadImage(void);
GLuint createTexture(image image);


Texture images_load()
{
	image  img         = loadImage();
	GLuint textureName = createTexture(img);

	Texture texture = {textureName, img.width, img.height};

	return texture;
}
