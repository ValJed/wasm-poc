import path from 'path'

export default {
  mode: process.env.NODE_ENV || 'development',
  target: 'web',
  entry: {
    main: './src/index.js',
    style: './src/main.css',
  },
  optimization: {
    minimize: process.env.NODE_ENV === 'production',
  },
  output: {
    filename: '[name][ext]',
    path: path.resolve('dist'),
  },
  resolve: {
    extensions: ['*', '.js', 'scss'],
    alias: {
      '@': './src',
      'wasm-lib': './wasm-lib/pkg/wasm_lib.js',
    },
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: [
          {
            loader: 'postcss-loader',
            options: {
              postcssOptions: {
                plugins: {
                  /* tailwindcss: {}, */
                  autoprefixer: {},
                },
              },
            },
          },
        ],
        type: 'css',
      },
    ],
  },
  devServer: {
    watchFiles: [
      /* 'index.html',  */
      'src/**',
      /* 'wasm-lib/src/**' */
    ],
  },
  builtins: {
    html: [
      {
        template: 'index.html',
      },
    ],
  },
  stats: 'verbose',
}
