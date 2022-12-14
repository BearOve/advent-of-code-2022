use crate::*;

pub use self::dynamic_image::*;

#[export_module]
mod dynamic_image {
    pub type SharedDynImg = Shared<Locked<DynamicImage>>;

    pub struct DynamicImage {
        width: usize,
        data: Vec<u32>,
    }

    impl DynamicImage {
        fn dim(&self) -> (usize, usize) {
            (self.width, self.data.len() / self.width)
        }

        fn calc_x_or_y(
            &self,
            ctx: &NativeCallContext,
            src: &str,
            prop: &str,
            max: usize,
            val: INT,
        ) -> RhaiRes<usize> {
            if val < 0 {
                let off: usize = try_from(ctx, val.abs())?;
                if off <= max {
                    return Ok(max - off);
                }
            } else {
                let i: usize = try_from(ctx, val)?;
                if i < max {
                    return Ok(i);
                }
            }
            Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("{src} ({val}) does not fit within {prop} ({max})").into(),
                ctx.position(),
            )))
        }

        fn calc_index(&self, ctx: &NativeCallContext, (x, y): (INT, INT)) -> RhaiRes<usize> {
            let (width, height) = self.dim();
            let x = self.calc_x_or_y(ctx, "x", "width", width, x)?;
            let y = self.calc_x_or_y(ctx, "y", "height", height, y)?;
            Ok((y * width) + x)
        }

        fn calc_rect(
            &self,
            ctx: &NativeCallContext,
            (left, top, right, bottom): (INT, INT, INT, INT),
        ) -> RhaiRes<Rect> {
            let (width, height) = self.dim();
            let right = self.calc_x_or_y(ctx, "right", "width", width, right)?;
            let left = self.calc_x_or_y(ctx, "left", "right", right, left)?;
            let bottom = self.calc_x_or_y(ctx, "bottom", "height", height, bottom)?;
            let top = self.calc_x_or_y(ctx, "top", "bottom", bottom, top)?;
            Ok(Rect {
                x: left,
                y: top,
                width: right - left,
                height: bottom - top,
                stride: width,
            })
        }
    }

    struct Rect {
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        stride: usize,
    }

    #[rhai_fn(return_raw)]
    pub fn dynamic_image(ctx: NativeCallContext, width: INT, height: INT) -> RhaiRes<SharedDynImg> {
        let width: usize = try_from(&ctx, width)?;
        let height: usize = try_from(&ctx, height)?;
        Ok(Shared::new(Locked::new(DynamicImage {
            width,
            data: vec![0; width * height],
        })))
    }

    #[rhai_fn(pure, get = "width", return_raw)]
    pub fn dynamic_image_width(ctx: NativeCallContext, img: &mut SharedDynImg) -> RhaiRes<INT> {
        try_from(&ctx, img.borrow().width)
    }

    #[rhai_fn(pure, get = "height", return_raw)]
    pub fn dynamic_image_height(ctx: NativeCallContext, img: &mut SharedDynImg) -> RhaiRes<INT> {
        let img = img.borrow();
        try_from(&ctx, img.data.len() / img.width)
    }

    #[rhai_fn(return_raw)]
    pub fn push_row(ctx: NativeCallContext, img: &mut SharedDynImg, row: Blob) -> RhaiRes<()> {
        let mut img = img.borrow_mut();
        if row.len() != img.width {
            return Err(mismatching_data_type(
                &ctx,
                format!("blob of len {}", img.width),
                format!("blob of len {}", row.len()),
            ));
        }
        img.data.extend(row.into_iter().map(u32::from));
        Ok(())
    }

    #[rhai_fn(return_raw)]
    pub fn pixel(ctx: NativeCallContext, img: SharedDynImg, x: INT, y: INT) -> RhaiRes<Pixel> {
        pixel_tup(ctx, img, (x, y))
    }

    #[rhai_fn(return_raw, name = "pixel")]
    pub fn pixel_by_index(ctx: NativeCallContext, img: SharedDynImg, i: INT) -> RhaiRes<Pixel> {
        let index = {
            let img = img.borrow();
            img.calc_x_or_y(&ctx, "index", "len", img.data.len(), i)?
        };
        Ok(Pixel { index, img })
    }

    pub fn pixels(img: SharedDynImg) -> DynIterator<Pixel> {
        let len = img.borrow().data.len();
        DynIterator::new((0..len).map(move |index| Pixel {
            index,
            img: img.clone(),
        }))
    }

    #[rhai_fn(name = "pixels", return_raw)]
    pub fn pixels_rect(
        ctx: NativeCallContext,
        img: SharedDynImg,
        left: INT,
        top: INT,
        right: INT,
        bottom: INT,
    ) -> RhaiRes<DynIterator<Pixel>> {
        let Rect {
            x,
            y,
            width,
            height,
            stride,
        } = img.borrow().calc_rect(&ctx, (left, top, right, bottom))?;

        Ok(DynIterator::new(
            (y..y + height)
                .flat_map(move |y| (x..x + width).map(move |x| (x, y)))
                .map(move |(x, y)| Pixel {
                    index: (y * stride) + x,
                    img: img.clone(),
                }),
        ))
    }

    #[rhai_fn(name = "pixel", return_raw)]
    pub fn pixel_tup(ctx: NativeCallContext, img: SharedDynImg, pos: (INT, INT)) -> RhaiRes<Pixel> {
        let index = img.borrow().calc_index(&ctx, pos)?;
        Ok(Pixel { index, img })
    }

    pub const DOMINO_UP: DominoDir = DominoDir::Up;
    pub const DOMINO_DOWN: DominoDir = DominoDir::Down;
    pub const DOMINO_LEFT: DominoDir = DominoDir::Left;
    pub const DOMINO_RIGHT: DominoDir = DominoDir::Right;

    #[rhai_fn(return_raw)]
    pub fn domino(
        ctx: NativeCallContext,
        img: SharedDynImg,
        x: INT,
        y: INT,
        dir: DominoDir,
    ) -> RhaiRes<Domino> {
        domino_tup(ctx, img, (x, y), dir)
    }

    #[rhai_fn(return_raw, name = "domino")]
    pub fn domino_tup(
        ctx: NativeCallContext,
        img: SharedDynImg,
        pos: (INT, INT),
        dir: DominoDir,
    ) -> RhaiRes<Domino> {
        let (a, b) = {
            let img = img.borrow();
            (
                img.calc_index(&ctx, dir.translate_pos(pos))?,
                img.calc_index(&ctx, pos)?,
            )
        };
        Ok(Domino { a, b, img })
    }

    fn iter_rows(
        img: SharedDynImg,
        start_y: usize,
        count: usize,
    ) -> impl DoubleEndedIterator<Item = Row> {
        let it = {
            let img = img.borrow();
            let width = img.width;
            (start_y..start_y + count.min(img.dim().1)).map(move |y| y * width)
        };
        it.map(move |start| Row {
            start,
            img: img.clone(),
        })
    }

    fn iter_cols(img: SharedDynImg) -> impl DoubleEndedIterator<Item = Col> {
        let it = 0..img.borrow().width;
        it.map(move |start| Col {
            start,
            img: img.clone(),
        })
    }

    pub fn rows(img: SharedDynImg) -> DynIterator<Row> {
        DynIterator::new(iter_rows(img, 0, usize::MAX))
    }

    #[rhai_fn(return_raw)]
    pub fn row(ctx: NativeCallContext, img: SharedDynImg, y: INT) -> RhaiRes<Row> {
        let start = {
            let img = img.borrow();
            let (width, height) = img.dim();
            img.calc_x_or_y(&ctx, "y", "height", height, y)? * width
        };
        Ok(Row { start, img })
    }

    #[rhai_fn(return_raw, name = "skip")]
    pub fn rows_skip(
        ctx: NativeCallContext,
        it: DynIterator<Row>,
        count: INT,
    ) -> RhaiRes<DynIterator<Row>> {
        it.into_skip(&ctx, count)
    }

    #[rhai_fn(return_raw, name = "skip")]
    pub fn pixels_skip(
        ctx: NativeCallContext,
        it: DynIterator<Pixel>,
        count: INT,
    ) -> RhaiRes<DynIterator<Pixel>> {
        it.into_skip(&ctx, count)
    }

    pub fn rrows(img: SharedDynImg) -> DynIterator<Row> {
        DynIterator::new(iter_rows(img, 0, usize::MAX).rev())
    }

    pub fn cols(img: SharedDynImg) -> DynIterator<Col> {
        DynIterator::new(iter_cols(img))
    }

    pub fn rcols(img: SharedDynImg) -> DynIterator<Col> {
        DynIterator::new(iter_cols(img).rev())
    }

    #[rhai_fn(pure, return_raw)]
    pub fn crop(
        ctx: NativeCallContext,
        img: &mut SharedDynImg,
        sx: INT,
        sy: INT,
        ex: INT,
        ey: INT,
    ) -> RhaiRes<SharedDynImg> {
        let img = img.borrow();
        let rect = img.calc_rect(&ctx, (sx, sy, ex, ey))?;
        let mut data = Vec::with_capacity(rect.width * rect.height);

        for row in img.data[rect.y * img.width..]
            .chunks(rect.stride)
            .take(rect.height)
        {
            data.extend_from_slice(&row[rect.x..rect.x + rect.width]);
        }

        Ok(Shared::new(Locked::new(DynamicImage {
            width: rect.width,
            data,
        })))
    }

    pub fn to_2bit_ascii_art(img: SharedDynImg) -> String {
        to_ascii_art(img, "_X")
    }

    pub fn to_ascii_art(img: SharedDynImg, color_map: &str) -> String {
        let color_map = color_map.as_bytes();
        let last = color_map.last().unwrap();
        assert!(color_map.is_ascii());

        let img = img.borrow();
        let (width, height) = img.dim();
        let mut ret = String::with_capacity((width + 1) * height);
        for row in img.data.chunks_exact(img.width) {
            for pix in row.iter().copied() {
                ret.push(char::from(*color_map.get(pix as usize).unwrap_or(last)));
            }
            ret.push('\n');
        }
        ret
    }

    #[derive(Clone)]
    pub struct Row {
        start: usize,
        img: SharedDynImg,
    }

    impl Row {
        fn iter(self) -> impl DoubleEndedIterator<Item = Pixel> {
            let width = self.img.borrow().width;
            (self.start..self.start + width).map(move |index| Pixel {
                index,
                img: self.img.clone(),
            })
        }
    }

    impl IntoIterator for Row {
        type Item = Pixel;
        type IntoIter = DynIterator<Pixel>;

        fn into_iter(self) -> Self::IntoIter {
            row_iter(self)
        }
    }

    #[rhai_fn(name = "iter")]
    pub fn row_iter(row: Row) -> DynIterator<Pixel> {
        DynIterator::new(row.iter())
    }

    #[rhai_fn(name = "rev_iter")]
    pub fn row_rev_iter(row: Row) -> DynIterator<Pixel> {
        DynIterator::new(row.iter().rev())
    }

    #[derive(Clone)]
    pub struct Col {
        start: usize,
        img: SharedDynImg,
    }

    impl Col {
        fn iter(self) -> impl DoubleEndedIterator<Item = Pixel> {
            let it = {
                let img = self.img.borrow();
                (self.start..img.data.len()).step_by(img.width)
            };
            it.map(move |index| Pixel {
                index,
                img: self.img.clone(),
            })
        }
    }

    impl IntoIterator for Col {
        type Item = Pixel;
        type IntoIter = DynIterator<Pixel>;

        fn into_iter(self) -> Self::IntoIter {
            col_iter(self)
        }
    }

    #[rhai_fn(name = "iter")]
    pub fn col_iter(col: Col) -> DynIterator<Pixel> {
        DynIterator::new(col.iter())
    }

    #[rhai_fn(name = "rev_iter")]
    pub fn col_rev_iter(col: Col) -> DynIterator<Pixel> {
        DynIterator::new(col.iter().rev())
    }

    #[derive(Clone)]
    pub struct Pixel {
        index: usize,
        img: SharedDynImg,
    }

    impl Pixel {
        fn raw_pos(&self) -> (usize, usize, usize, usize) {
            let img = self.img.borrow();
            (
                self.index % img.width,
                self.index / img.width,
                img.width,
                img.data.len(),
            )
        }

        fn pos(&self, ctx: &NativeCallContext) -> RhaiRes<(INT, INT)> {
            let (x, y, _, _) = self.raw_pos();
            Ok((try_from(ctx, x)?, try_from(ctx, y)?))
        }

        fn value(&self) -> INT {
            INT::from(self.img.borrow().data[self.index])
        }
    }

    impl Debug for Pixel {
        fn fmt(&self, f: &mut Formatter) -> FmtResult {
            let (x, y, _, _) = self.raw_pos();
            let value = self.value();
            f.debug_struct("Pixel")
                .field("x", &x)
                .field("y", &y)
                .field("value", &value)
                .finish()
        }
    }

    #[rhai_fn(name = "right_pixels")]
    pub fn pixel_right_pixels(mut pix: Pixel) -> DynIterator<Pixel> {
        let (x, _, width, _) = pix.raw_pos();

        DynIterator::new((x + 1..width).map(move |_| {
            pix.index += 1;
            pix.clone()
        }))
    }

    #[rhai_fn(name = "left_pixels")]
    pub fn pixel_left_pixels(mut pix: Pixel) -> DynIterator<Pixel> {
        let (x, _, _, _) = pix.raw_pos();

        DynIterator::new((0..x).map(move |_| {
            pix.index -= 1;
            pix.clone()
        }))
    }

    #[rhai_fn(name = "up_pixels")]
    pub fn pixel_up_pixels(mut pix: Pixel) -> DynIterator<Pixel> {
        let (_, y, width, _) = pix.raw_pos();

        DynIterator::new((0..y).map(move |_| {
            pix.index -= width;
            pix.clone()
        }))
    }

    #[rhai_fn(name = "down_pixels")]
    pub fn pixel_down_pixels(mut pix: Pixel) -> DynIterator<Pixel> {
        let (_, y, width, data_len) = pix.raw_pos();
        let height = data_len / width;

        DynIterator::new((y + 1..height).map(move |_| {
            pix.index += width;
            pix.clone()
        }))
    }

    #[rhai_fn(name = "ortho_pixels")]
    #[allow(clippy::unnecessary_lazy_evaluations)] // rust-clippy/issues/10071
    pub fn pixel_ortho_pixels(pix: Pixel) -> DynIterator<Pixel> {
        let (x, _, width, data_len) = pix.raw_pos();
        let tmp = [
            pix.index.checked_sub(width),
            (x + 1 < width).then(|| pix.index + 1),
            (x > 0).then(|| pix.index - 1),
            (pix.index + width < data_len).then(|| pix.index + width),
        ];

        DynIterator::new(tmp.into_iter().flatten().map(move |index| Pixel {
            index,
            img: pix.img.clone(),
        }))
    }

    #[rhai_fn(pure, get = "pos", return_raw)]
    pub fn pixel_position(ctx: NativeCallContext, pix: &mut Pixel) -> RhaiRes<(INT, INT)> {
        pix.pos(&ctx)
    }

    #[rhai_fn(pure, get = "x", return_raw)]
    pub fn pixel_get_x(ctx: NativeCallContext, pix: &mut Pixel) -> RhaiRes<INT> {
        try_from(&ctx, pix.index % pix.img.borrow().width)
    }

    #[rhai_fn(pure, get = "y", return_raw)]
    pub fn pixel_get_y(ctx: NativeCallContext, pix: &mut Pixel) -> RhaiRes<INT> {
        try_from(&ctx, pix.index / pix.img.borrow().width)
    }

    #[rhai_fn(pure, name = "as_int")]
    pub fn pixel_as_int(pix: &mut Pixel) -> INT {
        pix.img.borrow().data[pix.index].into()
    }

    #[rhai_fn(name = "set", return_raw)]
    pub fn pixel_set_bool(ctx: NativeCallContext, pix: &mut Pixel, rhs: bool) -> RhaiRes<INT> {
        pixel_apply_int(ctx, pix, |_| rhs.into())
    }

    #[rhai_fn(name = "set", return_raw)]
    pub fn pixel_set_int(ctx: NativeCallContext, pix: &mut Pixel, rhs: INT) -> RhaiRes<INT> {
        pixel_apply_int(ctx, pix, |_| rhs)
    }

    #[rhai_fn(name = "-=", return_raw)]
    pub fn pixel_sub_int(ctx: NativeCallContext, pix: &mut Pixel, rhs: INT) -> RhaiRes<()> {
        pixel_apply_int(ctx, pix, |lhs| lhs - rhs).map(|_| ())
    }

    #[rhai_fn(name = "+=", return_raw)]
    pub fn pixel_add_int(ctx: NativeCallContext, pix: &mut Pixel, rhs: INT) -> RhaiRes<()> {
        pixel_apply_int(ctx, pix, |lhs| lhs + rhs).map(|_| ())
    }

    #[rhai_fn(name = "|=", return_raw)]
    pub fn pixel_bitor_int(ctx: NativeCallContext, pix: &mut Pixel, rhs: INT) -> RhaiRes<()> {
        pixel_apply_int(ctx, pix, |lhs| lhs | rhs).map(|_| ())
    }

    #[rhai_fn(name = "&=", return_raw)]
    pub fn pixel_bitand_int(ctx: NativeCallContext, pix: &mut Pixel, rhs: INT) -> RhaiRes<()> {
        pixel_apply_int(ctx, pix, |lhs| lhs & rhs).map(|_| ())
    }

    #[rhai_fn(name = "move", return_raw)]
    pub fn pixel_move(
        ctx: NativeCallContext,
        pix: &mut Pixel,
        add_x: INT,
        add_y: INT,
    ) -> RhaiRes<()> {
        let (x, y) = pix.pos(&ctx)?;
        *pix = pixel(ctx, pix.img.clone(), x + add_x, y + add_y)?;
        Ok(())
    }

    #[rhai_fn(pure, name = "euc_dist", return_raw)]
    pub fn pixel_euc_dist(ctx: NativeCallContext, pix: &mut Pixel, other: Pixel) -> RhaiRes<FLOAT> {
        let (a_x, a_y) = pix.pos(&ctx)?;
        let (b_x, b_y) = other.pos(&ctx)?;
        let x_dist = (a_x - b_x) as FLOAT;
        let y_dist = (a_y - b_y) as FLOAT;
        Ok(((x_dist * x_dist) - (y_dist * y_dist)).abs().sqrt())
    }

    #[rhai_fn(pure, name = "diff", return_raw)]
    pub fn pixel_diff(
        ctx: NativeCallContext,
        pix: &mut Pixel,
        other: Pixel,
    ) -> RhaiRes<(INT, INT)> {
        let (a_x, a_y) = pix.pos(&ctx)?;
        let (b_x, b_y) = other.pos(&ctx)?;
        Ok(((a_x - b_x), (a_y - b_y)))
    }

    fn pixel_apply_int(
        ctx: NativeCallContext,
        pix: &mut Pixel,
        f: impl FnOnce(INT) -> INT,
    ) -> RhaiRes<INT> {
        let mut img = pix.img.borrow_mut();
        let pix = &mut img.data[pix.index];
        let pix_int = INT::from(*pix);
        *pix = try_from(&ctx, f(pix_int))?;
        Ok(pix_int)
    }

    #[rhai_fn(pure, name = "to_debug")]
    pub fn pixel_to_debug(pix: &mut Pixel) -> String {
        format!("{:?}", pix)
    }

    #[derive(Clone, Copy)]
    pub enum DominoDir {
        Up,
        Down,
        Left,
        Right,
    }

    impl DominoDir {
        fn translate_pos(self, pos: (INT, INT)) -> (INT, INT) {
            match self {
                DominoDir::Up => (pos.0, pos.1 - 1),
                DominoDir::Down => (pos.0, pos.1 + 1),
                DominoDir::Left => (pos.0 - 1, pos.1),
                DominoDir::Right => (pos.0 + 1, pos.1),
            }
        }
    }

    #[derive(Clone)]
    pub struct Domino {
        a: usize,
        b: usize,
        img: SharedDynImg,
    }

    impl Domino {
        fn pixels(&self) -> (Pixel, Pixel) {
            (
                Pixel {
                    index: self.a,
                    img: self.img.clone(),
                },
                Pixel {
                    index: self.b,
                    img: self.img.clone(),
                },
            )
        }
    }

    #[rhai_fn(return_raw, index_get)]
    pub fn domino_index_tup(ctx: NativeCallContext, tup: Domino, index: INT) -> RhaiRes<Pixel> {
        index_tup2(ctx, (tup.a, tup.b), index).map(|index| Pixel {
            index,
            img: tup.img,
        })
    }

    #[rhai_fn(pure, name = "to_debug")]
    pub fn domino_to_debug(tup: &mut Domino) -> String {
        format!("{:?}", tup.pixels())
    }
}
