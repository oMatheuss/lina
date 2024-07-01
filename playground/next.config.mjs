import MonacoWebpackPlugin from 'monaco-editor-webpack-plugin';

/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  basePath: process.env.BASE_PATH,
  webpack: (config, _ctx) => {
    config.plugins.push(new MonacoWebpackPlugin({
      filename: 'static/[name].worker.[contenthash].js',
      publicPath: (process.env.BASE_PATH ?? '') + '/_next',
      languages: [],
    }));
    return config;
  },
  transpilePackages: ['file:../lina-wasm/pkg', 'monaco-editor'],
  images: { unoptimized: true } 
};

export default nextConfig;
