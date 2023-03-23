const path = require('path');
const fs = require('fs');

const srcAlias = {
  "src": path.resolve(__dirname, './src'),
};

const dirs = fs.readdirSync('./src');
dirs.forEach(dir => {
  srcAlias['@' + dir] = path.resolve(__dirname, './src', dir);
});


const emitSourceMap = process.env.NODE_ENV === 'production' ? false : 'source-map';
const pages = fs.readdirSync('./src/isolate-pages');
const outputDir = path.resolve(__dirname, '../server/static');

if (fs.existsSync(outputDir)) {
  fs.rmSync(outputDir, { recursive: true, force: true });
}

const entries = pages.reduce((prev, file) => {
  let entryName = file.split('.')[0];
  return {
    ...prev,
    [entryName]: path.join('./src/isolate-pages', file),
  }
}, {
  main: './src/index.tsx',
});

const htmlWithChuncks = [
  {
    chunks: ['login'],
    filename: 'login.html',
    template: './public/index.html'
  },
  {
    chunks: ['main'],
    filename: 'index.html',
    template: './public/index.html'
  }
];

module.exports = {
  entry: entries,
  output: {
    filename: '[name].js',
    path: outputDir,
  },
  resolve: {
    alias: {
      ...srcAlias
    }
  },
  module: {
    rules: [
      {
        test: /\.module\.less$/,
        type: "css/module",
        use: [
          {
            loader: 'less-loader',
            options: {
              lessOptions: {

              },
            },
          },
        ],
      },
      {
        test: /\.less$/,
        exclude: /\.module\.less$/,
        use: [
          {
            loader: 'less-loader',
            options: {
              lessOptions: {

              },
            },
          },
        ],
        type: 'css',
      },
    ],
  },
  builtins: {
    html: htmlWithChuncks,
    define: {
      'import.meta.env': "{}",
    },
  },
  devtool: emitSourceMap,
  devServer: {
    proxy: {
      '/file': {
        target: 'http://127.0.0.1:7001',
        changeOrigin: true,
      },
      '/auth': {
        target: 'http://127.0.0.1:7001',
        changeOrigin: true,
      },
    },
  },
};