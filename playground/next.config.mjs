import MonacoWebpackPlugin from 'monaco-editor-webpack-plugin';

/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  webpack: (config, _ctx) => {
    config.plugins.push(new MonacoWebpackPlugin({
      filename: 'static/[name].worker.[contenthash].js',
      publicPath: '/_next',
      languages: [],
    }));
    return config;
  },
  transpilePackages: ['file:../lina-wasm/pkg', 'monaco-editor'],
  images: { unoptimized: true } 
};

export default nextConfig;
