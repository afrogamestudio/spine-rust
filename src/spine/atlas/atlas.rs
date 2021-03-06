use super::AtlasRegion;
use super::AtlasPage;

pub struct Atlas// : IEnumerable<AtlasRegion> {
{
    pub regions : Vec<AtlasRegion>,
    pub pages: Vec<AtlasPage>

    //TextureLoader textureLoader;
}

impl Atlas
{
    // private async Task ReadFile(string path, TextureLoader textureLoader) {
    //     var folder = Windows.ApplicationModel.Package.Current.InstalledLocation;
    //     var file = await folder.GetFileAsync(path).AsTask().ConfigureAwait(false);
    //     using (var reader = new StreamReader(await file.OpenStreamForReadAsync().ConfigureAwait(false))) {
    //         try {
    //             Load(reader, Path.GetDirectoryName(path), textureLoader);
    //         } catch (Exception ex) {
    //             throw new Exception("Error reading atlas file: " + path, ex);
    //         }
    //     }
    // }

    // pub Atlas(string path, TextureLoader textureLoader) {
    //     this.ReadFile(path, textureLoader).Wait();
    // }
    // #else

    // pub Atlas (string path, TextureLoader textureLoader) {

    //     #if WINDOWS_PHONE
    //     Stream stream = Microsoft.Xna.Framework.TitleContainer.OpenStream(path);
    //     using (StreamReader reader = new StreamReader(stream)) {
    //     #else
    //     using (StreamReader reader = new StreamReader(path)) {
    //     #endif // WINDOWS_PHONE

    //         try {
    //             Load(reader, Path.GetDirectoryName(path), textureLoader);
    //         } catch (Exception ex) {
    //             throw new Exception("Error reading atlas file: " + path, ex);
    //         }

    //     }
    // }
    // #endif // WINDOWS_STOREAPP

    // #endif

    // pub Atlas (TextReader reader, string dir, TextureLoader textureLoader) {
    //     Load(reader, dir, textureLoader);
    // }

    // pub Atlas (List<AtlasPage> pages, List<AtlasRegion> regions) {
    //     this.pages = pages;
    //     this.regions = regions;
    //     this.textureLoader = null;
    // }

    // private void Load (TextReader reader, string imagesDir, TextureLoader textureLoader) {
    //     if (textureLoader == null) throw new ArgumentNullException("textureLoader", "textureLoader cannot be null.");
    //     this.textureLoader = textureLoader;

    //     string[] tuple = new string[4];
    //     AtlasPage page = null;
    //     while (true) {
    //         string line = reader.ReadLine();
    //         if (line == null) break;
    //         if (line.Trim().Length == 0)
    //             page = null;
    //         else if (page == null) {
    //             page = new AtlasPage();
    //             page.name = line;

    //             if (ReadTuple(reader, tuple) == 2) { // size is only optional for an atlas packed with an old TexturePacker.
    //                 page.width = int.Parse(tuple[0]);
    //                 page.height = int.Parse(tuple[1]);
    //                 ReadTuple(reader, tuple);
    //             }
    //             page.format = (Format)Enum.Parse(typeof(Format), tuple[0], false);

    //             ReadTuple(reader, tuple);
    //             page.minFilter = (TextureFilter)Enum.Parse(typeof(TextureFilter), tuple[0], false);
    //             page.magFilter = (TextureFilter)Enum.Parse(typeof(TextureFilter), tuple[1], false);

    //             string direction = ReadValue(reader);
    //             page.uWrap = TextureWrap.ClampToEdge;
    //             page.vWrap = TextureWrap.ClampToEdge;
    //             if (direction == "x")
    //                 page.uWrap = TextureWrap.Repeat;
    //             else if (direction == "y")
    //                 page.vWrap = TextureWrap.Repeat;
    //             else if (direction == "xy")
    //                 page.uWrap = page.vWrap = TextureWrap.Repeat;

    //             textureLoader.Load(page, Path.Combine(imagesDir, line));

    //             pages.Add(page);

    //         } else {
    //             AtlasRegion region = new AtlasRegion();
    //             region.name = line;
    //             region.page = page;

    //             region.rotate = Boolean.Parse(ReadValue(reader));

    //             ReadTuple(reader, tuple);
    //             int x = int.Parse(tuple[0]);
    //             int y = int.Parse(tuple[1]);

    //             ReadTuple(reader, tuple);
    //             int width = int.Parse(tuple[0]);
    //             int height = int.Parse(tuple[1]);

    //             region.u = x / (f64)page.width;
    //             region.v = y / (f64)page.height;
    //             if (region.rotate) {
    //                 region.u2 = (x + height) / (f64)page.width;
    //                 region.v2 = (y + width) / (f64)page.height;
    //             } else {
    //                 region.u2 = (x + width) / (f64)page.width;
    //                 region.v2 = (y + height) / (f64)page.height;
    //             }
    //             region.x = x;
    //             region.y = y;
    //             region.width = Math.Abs(width);
    //             region.height = Math.Abs(height);

    //             if (ReadTuple(reader, tuple) == 4) { // split is optional
    //                 region.splits = new [] {int.Parse(tuple[0]), int.Parse(tuple[1]),
    //                         int.Parse(tuple[2]), int.Parse(tuple[3])};

    //                 if (ReadTuple(reader, tuple) == 4) { // pad is optional, but only present with splits
    //                     region.pads = new [] {int.Parse(tuple[0]), int.Parse(tuple[1]),
    //                             int.Parse(tuple[2]), int.Parse(tuple[3])};

    //                     ReadTuple(reader, tuple);
    //                 }
    //             }

    //             region.originalWidth = int.Parse(tuple[0]);
    //             region.originalHeight = int.Parse(tuple[1]);

    //             ReadTuple(reader, tuple);
    //             region.offsetX = int.Parse(tuple[0]);
    //             region.offsetY = int.Parse(tuple[1]);

    //             region.index = int.Parse(ReadValue(reader));

    //             regions.Add(region);
    //         }
    //     }
    // }

    // static string ReadValue (TextReader reader) {
    //     string line = reader.ReadLine();
    //     int colon = line.IndexOf(':');
    //     if (colon == -1) throw new Exception("Invalid line: " + line);
    //     return line.Substring(colon + 1).Trim();
    // }

    // /// <summary>Returns the number of tuple values read (1, 2 or 4).</summary>
    // static int ReadTuple (TextReader reader, string[] tuple) {
    //     string line = reader.ReadLine();
    //     int colon = line.IndexOf(':');
    //     if (colon == -1) throw new Exception("Invalid line: " + line);
    //     int i = 0, lastMatch = colon + 1;
    //     for (; i < 3; i++) {
    //         int comma = line.IndexOf(',', lastMatch);
    //         if (comma == -1) break;
    //         tuple[i] = line.Substring(lastMatch, comma - lastMatch).Trim();
    //         lastMatch = comma + 1;
    //     }
    //     tuple[i] = line.Substring(lastMatch).Trim();
    //     return i + 1;
    // }

    // pub void FlipV () {
    //     for (int i = 0, n = regions.Count; i < n; i++) {
    //         AtlasRegion region = regions[i];
    //         region.v = 1 - region.v;
    //         region.v2 = 1 - region.v2;
    //     }
    // }

    // /// <summary>Returns the first region found with the specified name. This method uses string comparison to find the region, so the result
    // /// should be cached rather than calling this method multiple times.</summary>
    // /// <returns>The region, or null.</returns>
    // pub AtlasRegion FindRegion (string name) {
    //     for (int i = 0, n = regions.Count; i < n; i++)
    //         if (regions[i].name == name) return regions[i];
    //     return null;
    // }

    // pub void Dispose () {
    //     if (textureLoader == null) return;
    //     for (int i = 0, n = pages.Count; i < n; i++)
    //         textureLoader.Unload(pages[i].rendererObject);
    // }
}