## axum

### middleware
- To take and set cookies it need middleware
- Mostly middleware flow is top to bottom
  - So in order to get cookies to other layer then it should be place at bottom.

* Route::nest("PREFIX_path", "ROUTE/ROUTES")
- Here the all routes (sended in second args) are added prefix path (path = `"PREFIX_path" + "ROUTE_PATH"`)

* `FromRef trait`
