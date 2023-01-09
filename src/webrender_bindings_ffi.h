typedef struct ns_output output;

extern void syms_of_wrfns (void);
extern int wr_get_fontset(output* output);
extern struct font *wr_get_font(output* output);
extern int wr_get_baseline_offset(output* output);
extern int wr_get_pixel(WRImage *ximg, int x, int y);
extern int wr_put_pixel(WRImage *ximg, int x, int y, unsigned long pixel);
extern bool wr_load_image (struct frame *f, struct image *img,
			   Lisp_Object spec_file, Lisp_Object spec_data);
extern bool wr_can_use_native_image_api (Lisp_Object type);
extern void wr_transform_image(struct frame *f, struct image *img, int width, int height, double rotation);

#define FRAME_FONTSET(f) (wr_get_fontset(FRAME_X_OUTPUT (f)))
#define FRAME_FONT(f) (wr_get_font(FRAME_X_OUTPUT (f)))
#define FRAME_BASELINE_OFFSET(f) (wr_get_baseline_offset(FRAME_X_OUTPUT (f)))

#define BLACK_PIX_DEFAULT(f) 0
#define WHITE_PIX_DEFAULT(f) 65535
