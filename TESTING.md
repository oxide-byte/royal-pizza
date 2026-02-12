# Royal Pizza - Testing Guide

## Phase 7 Edge Case Testing Documentation

This document outlines all edge cases and error scenarios that need to be tested to ensure the application handles failures gracefully.

---

## Test Environment Setup

### Prerequisites
1. Docker and Docker Compose installed
2. All services running via `docker-compose up`
3. Browser DevTools open for network inspection

### Service URLs
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080/api
- Database: ws://localhost:8000 (SurrealDB)

---

## Test Scenarios Matrix

### 1. Network Failures

| Test Case | Steps | Expected Behavior | Status |
|-----------|-------|-------------------|--------|
| **Menu load with network down** | 1. Disconnect network<br>2. Navigate to `/` | Show error message: "Unable to connect to the server. Please check your internet connection and try again."<br>Display ErrorDisplay component | ⬜ |
| **Order submission with network down** | 1. Add items to cart<br>2. Fill order form<br>3. Disconnect network<br>4. Submit order | Show error message with network failure<br>Button remains enabled for retry<br>Cart data preserved | ⬜ |
| **Confirmation page with network down** | 1. Disconnect network<br>2. Navigate to `/confirmation/{id}` | Show error message and "Back to Menu" button | ⬜ |

### 2. Backend Down Scenarios

| Test Case | Steps | Expected Behavior | Status |
|-----------|-------|-------------------|--------|
| **Backend not running** | 1. Stop backend container<br>2. Navigate to menu | Show network error message<br>LoadingSpinner shows then ErrorDisplay | ⬜ |
| **Backend crashes during order** | 1. Start order submission<br>2. Stop backend mid-request | Show appropriate error message<br>Allow retry<br>Data not corrupted | ⬜ |

### 3. Database Down Scenarios

| Test Case | Steps | Expected Behavior | Status |
|-----------|-------|-------------------|--------|
| **Database offline** | 1. Stop database container<br>2. Try to fetch pizzas | Backend returns 500 error<br>Frontend shows "Server error. Please try again later." | ⬜ |
| **Database connection lost** | 1. Disconnect database during operation<br>2. Try various operations | Graceful error handling<br>No application crash | ⬜ |

### 4. Invalid Data Scenarios

| Test Case | Steps | Expected Behavior | Status |
|-----------|-------|-------------------|--------|
| **Invalid order ID** | 1. Navigate to `/confirmation/invalid-id-123` | Show 404 error: "The requested resource was not found."<br>"Back to Menu" button visible | ⬜ |
| **Malformed order ID** | 1. Navigate to `/confirmation/!@#$%` | Handle gracefully with error message | ⬜ |
| **Non-existent pizza ID** | 1. Manually add non-existent pizza ID to cart state<br>2. Try to submit order | Backend validates and returns 400 error<br>Frontend shows validation message | ⬜ |

### 5. Validation Edge Cases

| Test Case | Steps | Expected Behavior | Status |
|-----------|-------|-------------------|--------|
| **Empty customer name** | 1. Leave name field blank<br>2. Submit order | Show validation error: "Name is required (2-100 characters)" | ⬜ |
| **Name too short (1 char)** | 1. Enter "A"<br>2. Submit order | Show validation error | ⬜ |
| **Name too long (>100 chars)** | 1. Enter 101+ character name<br>2. Submit order | Show validation error | ⬜ |
| **Empty phone number** | 1. Leave phone blank<br>2. Submit order | Show validation error: "Phone number is required" | ⬜ |
| **Pickup time in past** | 1. Set pickup time to yesterday<br>2. Submit order | Show validation error: "Pickup time must be at least 30 minutes from now" | ⬜ |
| **Pickup time < 30 min** | 1. Set pickup time to 15 minutes from now<br>2. Submit order | Show validation error about minimum lead time | ⬜ |
| **Custom pizza instructions too short (<10 chars)** | 1. Enter "Pizza"<br>2. Try to add to cart | Show validation error and disable button | ⬜ |
| **Custom pizza instructions too long (>500 chars)** | 1. Enter 501+ characters<br>2. Try to add to cart | Show character counter warning<br>Prevent submission | ⬜ |

### 6. Empty Cart Scenarios

| Test Case | Steps | Expected Behavior | Status |
|-----------|-------|-------------------|--------|
| **Access order page with empty cart** | 1. Clear cart<br>2. Navigate to `/order` | Automatically redirect to `/` (menu) | ⬜ |
| **Cart becomes empty during checkout** | 1. Add items<br>2. Go to order page<br>3. Remove all items | Redirect to menu OR show "Cart is empty" message | ⬜ |
| **Submit order with empty cart** | 1. Manipulate state to bypass redirect<br>2. Try to submit | Validation error: "Cart is empty. Please add items before ordering." | ⬜ |

### 7. Concurrent Operations

| Test Case | Steps | Expected Behavior | Status |
|-----------|-------|-------------------|--------|
| **Multiple rapid "Add to Cart" clicks** | 1. Quickly click "Add to Cart" 5 times | Each click increments quantity correctly<br>No duplicate items<br>Cart state consistent | ⬜ |
| **Update quantity while submitting order** | 1. Start order submission<br>2. Quickly update cart quantity | Order submitted with original quantities<br>OR cart locked during submission | ⬜ |

### 8. Browser Edge Cases

| Test Case | Steps | Expected Behavior | Status |
|-----------|-------|-------------------|--------|
| **Refresh on confirmation page** | 1. Complete order<br>2. Refresh `/confirmation/{id}` | Order details reload successfully<br>Cart remains cleared | ⬜ |
| **Back button after order** | 1. Complete order<br>2. Click browser back button | Return to menu<br>Cart is empty (not restored) | ⬜ |
| **Bookmark confirmation page** | 1. Complete order<br>2. Bookmark confirmation URL<br>3. Close browser<br>4. Open bookmark | Order details load correctly | ⬜ |

### 9. Stale Data Scenarios

| Test Case | Steps | Expected Behavior | Status |
|-----------|-------|-------------------|--------|
| **Pizza becomes unavailable** | 1. Load menu<br>2. Admin sets pizza to unavailable<br>3. Try to add to cart | Frontend still allows (cached data)<br>Backend validates on order submission | ⬜ |
| **Price changes after adding to cart** | 1. Add pizza to cart<br>2. Price changes in DB<br>3. Submit order | Order uses the price at time of submission<br>Backend recalculates total | ⬜ |

### 10. API Response Edge Cases

| Test Case | Steps | Expected Behavior | Status |
|-----------|-------|-------------------|--------|
| **API returns empty pizza list** | 1. Clear all pizzas from DB<br>2. Load menu | Show "No pizzas available" message<br>Custom pizza still available | ⬜ |
| **API returns malformed JSON** | 1. Configure API to return invalid JSON<br>2. Try to fetch pizzas | Show parse error: "Failed to process server response. Please try again." | ⬜ |
| **API returns 500 error** | 1. Configure backend to return 500<br>2. Try various operations | Show "Server error. Please try again later." | ⬜ |

### 11. Performance Edge Cases

| Test Case | Steps | Expected Behavior | Status |
|-----------|-------|-------------------|--------|
| **Very large cart (50+ items)** | 1. Add 50+ items to cart<br>2. Navigate to order page<br>3. Submit order | Page loads without lag<br>Scrolling smooth<br>Order submits successfully | ⬜ |
| **Long custom pizza instructions (500 chars)** | 1. Enter maximum length instructions<br>2. Add to cart<br>3. Submit order | No truncation<br>Order saved correctly | ⬜ |

### 12. Responsive Design Testing

| Test Case | Steps | Expected Behavior | Status |
|-----------|-------|-------------------|--------|
| **Mobile portrait (375px)** | 1. Resize browser to 375px width<br>2. Test all pages | 1-column pizza grid<br>Touch-friendly buttons (44px+)<br>Cart summary stacks vertically | ⬜ |
| **Tablet portrait (768px)** | 1. Resize to 768px<br>2. Test all pages | 2-column pizza grid<br>Forms readable<br>No horizontal scroll | ⬜ |
| **Desktop (1280px+)** | 1. Resize to 1280px<br>2. Test all pages | 3-column pizza grid<br>Optimal spacing<br>Content centered | ⬜ |

---

## Manual Testing Checklist

### Happy Path
- [ ] Browse menu → Add 2 pizzas → Add custom pizza → View cart
- [ ] Navigate to order page → Fill form with valid data → Submit
- [ ] View confirmation page → Check order number, items, total
- [ ] Click "Order Another" → Return to menu → Cart is empty

### Error Paths
- [ ] Test each validation error (name, phone, pickup time)
- [ ] Test network disconnection during each API call
- [ ] Test with backend stopped
- [ ] Test with database stopped
- [ ] Test invalid order ID in confirmation URL

### Edge Cases
- [ ] Empty cart access
- [ ] Rapid button clicking
- [ ] Browser back/forward buttons
- [ ] Page refresh on each route
- [ ] Very long customer names (99 characters)
- [ ] Custom pizza with exactly 10 and 500 characters

---

## Performance Benchmarks

| Metric | Target | Actual | Status |
|--------|--------|--------|--------|
| **Menu page load time** | < 2s | TBD | ⬜ |
| **API: GET /pizzas** | < 200ms | TBD | ⬜ |
| **API: POST /orders** | < 500ms | TBD | ⬜ |
| **API: GET /orders/:id** | < 200ms | TBD | ⬜ |
| **WASM bundle size** | < 1MB | TBD | ⬜ |
| **Total page weight** | < 3MB | TBD | ⬜ |

---

## Testing with Docker Compose

### Start all services
```bash
cd royal-pizza
docker-compose up --build
```

### Simulate backend failure
```bash
docker-compose stop backend
# Test frontend behavior
docker-compose start backend
```

### Simulate database failure
```bash
docker-compose stop database
# Test backend/frontend behavior
docker-compose start database
```

### View logs
```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f backend
docker-compose logs -f frontend
docker-compose logs -f database
```

### Restart with fresh data
```bash
docker-compose down -v  # Remove volumes
docker-compose up --build
```

---

## Accessibility Testing

### Keyboard Navigation
- [ ] Tab through all interactive elements in order
- [ ] Focus visible on all focusable elements
- [ ] Enter/Space activates buttons
- [ ] Escape closes modals/toasts (if applicable)

### Screen Reader
- [ ] All images have alt text (or use decorative role)
- [ ] Form labels properly associated
- [ ] Error messages announced
- [ ] Success messages announced

### Color Contrast
- [ ] Text meets WCAG AA standards (4.5:1 for normal text)
- [ ] Interactive elements distinguishable
- [ ] Error states clearly visible

---

## Known Limitations

1. **Timeout Handling**: gloo_net uses default browser timeouts. Very slow connections may not show a clear timeout message.

2. **Optimistic Updates**: Cart updates are immediate (optimistic). If backend fails during order submission, cart is already cleared.

3. **Stale Data**: Frontend caches pizza data until page refresh. Price changes during checkout won't reflect until backend validation.

4. **No Retry Logic**: Failed API calls don't automatically retry. User must manually retry.

5. **Session Persistence**: Cart is in-memory only. Refreshing page clears cart.

---

## Bug Reporting Template

When reporting issues found during testing:

```markdown
**Test Case**: [Name of test case from above]

**Steps to Reproduce**:
1.
2.
3.

**Expected Behavior**:


**Actual Behavior**:


**Environment**:
- Browser:
- OS:
- Docker Compose version:

**Screenshots/Logs**:
[Attach if applicable]

**Severity**: [Critical/High/Medium/Low]
```

---

## Test Status Legend

- ⬜ Not tested
- ✅ Passed
- ❌ Failed
- ⚠️ Partial pass (see notes)

---

**Last Updated**: 2026-02-12
**Tested By**: [Your Name]
**Test Environment**: Docker Compose (local)