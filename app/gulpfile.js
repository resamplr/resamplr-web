var gulp = require('gulp');
var elm = require('gulp-elm');
var minify = require('gulp-minify');
var rename = require('gulp-rename');
var plumber = require('gulp-plumber');
var del = require('del');
var concat = require('gulp-concat');
var stylus = require('gulp-stylus');
var webserver = require('gulp-webserver');

// builds elm files and static resources (i.e. html and css) 
// from src to public folder
var paths = {
 elm: 'elm/Main.elm',
 staticAssets: 'static/**/*.{html,css,js}',
 // where our compiled assets should sit
 // remember to .gitignore!
 dest: 'dist',
 stylus: 'styles/**/*.styl'
};

// clear all files from our dist folder
gulp.task('clean', function(cb) {
    del([paths.dest], cb);
});

gulp.task('elm-init', elm.init);

// build all elm files
gulp.task('elm', ['elm-init'], function() {
    return gulp.src(paths.elm)
        .pipe(plumber())
        .pipe(elm())
        .pipe(rename("main.js"))
        .pipe(minify({noSource: true}))
        .pipe(gulp.dest(paths.dest + "/js/"));
});

// compile stylus css 
gulp.task('stylus', function() {
  return gulp.src(paths.stylus)
    .pipe(plumber())
    .pipe(stylus())
    .pipe(concat('main.css'))
    .pipe(gulp.dest(paths.dest + "/css/"));
});

// compile static assets
gulp.task('staticAssets', function() {
  return gulp.src(paths.staticAssets)
    .pipe(plumber())
    .pipe(gulp.dest(paths.dest));
});

gulp.task('watch', function() {
  gulp.watch(paths.elm, ['elm']);
  gulp.watch(paths.staticAssets, ['static']);
  gulp.watch(paths.stylus, ['stylus']);
});

// spawn a small test server for our application
gulp.task('webserver', function() {
  gulp.src('dist/')
    .pipe(webserver({
      livereload: true,
      directoryListing: false,
      open: false
    }));
});


gulp.task('build', ['elm', 'staticAssets', 'stylus']);
gulp.task('dev', ['build', 'watch']);
gulp.task('default', ['build']);
gulp.task('serve', ['build', 'webserver', 'watch']);