import React, { createContext, useEffect, useState } from 'react';
import ReactDOM from 'react-dom/client';
import Index from './pages/index';
import reportWebVitals from './reportWebVitals';
import { BrowserRouter } from 'react-router-dom';
import { Navigate, Route, Routes } from 'react-router';
import Migrations from './pages/migrations';
import Login from './pages/login/login';
import Container from './template/container';
import {  notification } from 'antd';
import toast, { Toaster } from 'react-hot-toast';
import Cookies from 'universal-cookie';
import Profile from './pages/profile/profile';


const root = ReactDOM.createRoot(
	document.getElementById('root') as HTMLElement
);
	
	// create conetex for types const [isLoggedin, setIsLoggedin] = useState(false);
export const ctx = createContext({isLoggedin: false, setIsLoggedin: (value: boolean) => {}});
const App = () => {
    const cookies = new Cookies();
	const [isLoggedin, setIsLoggedin] = useState(cookies.get('token') !== undefined);	
	const [cachedLoggedIn, setCachedLoggedIn] = useState(isLoggedin);
	const [api, contextHolder] = notification.useNotification();

	useEffect(() => {
		// if hasnt changed 
		if (cachedLoggedIn === isLoggedin) {
			return;
		}

		setTimeout(() => {
		// if has changed
		setCachedLoggedIn(isLoggedin);
		// show antd toast
		if (isLoggedin) {
			toast.success('Logged in');
		} else {
			toast('You have been logged out.');
		}
		}, 500);
	}, [isLoggedin]);



	return (
		<ctx.Provider value={{isLoggedin, setIsLoggedin}} >
		<Toaster
		position="bottom-left"
		reverseOrder={false}
		/>
		<BrowserRouter>
			<Container>
			{!isLoggedin && <Login />}
			{isLoggedin && <Routes>
					<Route path="/login" element={<Login />} />
					<Route path="/" element={<Index />} />
					<Route path="/migrations" element={<Migrations />} />
					<Route path="/profile" element={<Profile />} >
						<Route path="*" element={<Profile />} />
					</Route>

					{/* 404 */}
					<Route path="*" element={<h1>404</h1>} />
				</Routes>
		}
			</Container>

		</BrowserRouter>
		</ctx.Provider>
	)
}

root.render(
	<React.StrictMode>
		{/* reactrouter */}
		<App/>
	</React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
