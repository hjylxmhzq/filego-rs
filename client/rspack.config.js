const path = require('path');
const fs = require('fs');

const srcAlias = {
  "src": path.resolve(__dirname, './src'),
};

const dirs = fs.readdirSync('./src');
dirs.forEach(dir => {
  srcAlias['@' + dir] = path.resolve(__dirname, './src', dir);
});

module.exports = {
  entry: {
    main: './src/index.tsx',
  },
  output: {
    filename: 'index.js',
    path: path.resolve(__dirname, '../server/static'),
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
    html: [{ template: './public/index.html' }],
    define: {
      'import.meta.env': "{}",
    },
  },
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