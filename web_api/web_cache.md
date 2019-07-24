Caching is one of the most important way to improve the response speed of Web Applications. 
Traditionally, Http caching is the one way to do it. Besides, SW is newer and more powerful.

Their order is as below:

<img src="../assets/cache_order.png" width="400"/>

## Http caching

There is no way to directly control Browser Cache from JavaScript. In my understanding, Http caching is to 
use Http headers to control it. There are some steps how browsers to do it:

1. search in Browser cache

First, the browser will find in its cache by the file's name and cache mark. If found, browser will directly return
the file, if not, browser send a request to the server.

For example, Chrome will get the cache from the memory or disk like below:

<img src="../assets/http_cache_chrome1.png" width="400"/>

There are some headers can be cache mark:

### Expires

From Http/1.0.

The Expires header contains the date/time after which the response is considered stale.

Invalid dates, like the value 0, represent a date in the past and mean that the resource is already expired.

If there is a Cache-Control header with the "max-age" or "s-max-age" directive in the response, the Expires header is ignored.

### Cache-Control

From Http/1.1.

The Cache-Control general-header field is used to specify directives for caching mechanisms in both requests and responses. Caching directives are unidirectional, meaning that a given directive in a request is not implying that the same directive is to be given in the response.

There are some common value:

> public: Indicates that the response may be cached by any cache. (both the client and the server proxy)<br>
> private: only the single user can use it.<br>
> no-cache: Forces caches to submit the request to the origin server for validation before releasing a cached copy.<br>
> max-age=<seconds>: Specifies the maximum amount of time a resource will be considered fresh. Contrary to Expires, this directive is relative to the time of the request.<br>

2. check cache with server

If the browser cannot find the resource's name, of course it will send request to the server. 

But, if the browser can find the file, but its cache mark shows it has already expired, the browser will send a request with the cache information of the resource.
The server will make the decision whether to use cached resource. 

There is a vivid picture to describe it:



