import setuptools

setuptools.setup(
    name="cyberpunk_display",
    version="0.0.1",
    author='xu-lai-xi',
    author_email='xu-lai-xi@qq.com',
    packages=setuptools.find_packages(),
    entry_points={
        'console_scripts': [
            'cyberpunk_display = cyberpunk_display.cli:main',
        ],
    },
)
