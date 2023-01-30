/*
  public void Render(WritableImage image) {
    //Get image dimensions, and declare loop variables
    int w = (int) image.getWidth(), h = (int) image.getHeight(), i, j;
    PixelWriter image_writer = image.getPixelWriter();
    double c = green_col / 255.0;
    Vector col = new Vector(0.5, c, 0.5);
    for (j = 0; j < h; j++) {
      for (i = 0; i < w; i++) {
        image_writer.setColor(i, j, Color.color(col.x, col.y, col.z, 1.0));
      } // column loop
    } // row loop
  }
 */