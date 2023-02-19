module.exports = {
    async redirects() {
      return [
            {
                source: '/api/:path*',
                destination: 'http://localhost:8080/api/:path*',
                permanent: true,
            },
        ]
    },
  }