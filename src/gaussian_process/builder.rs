use nalgebra::{DVector, DMatrix};
use crate::parameters::kernel::Kernel;
use crate::parameters::prior::Prior;
use super::GaussianProcess;
use crate::conversion::Input;

/// Builder to set the parameters of a gaussian process.
///
/// This class is meant to be produced by the `builder` method of the gaussian process and can be used to select the various parameters of the gaussian process :
///
/// ```rust
/// # use friedrich::gaussian_process::GaussianProcess;
/// # use friedrich::prior::*;
/// # use friedrich::kernel::*;
/// # fn main() {
/// // training data
/// let training_inputs = vec![vec![0.8], vec![1.2], vec![3.8], vec![4.2]];
/// let training_outputs = vec![3.0, 4.0, -2.0, -2.0];
///
/// // model parameters
/// let input_dimension = 1;
/// let output_noise = 0.1;
/// let exponential_kernel = Exponential::default();
/// let linear_prior = LinearPrior::default(input_dimension);
///
/// // defining and training a model
/// let gp = GaussianProcess::builder(training_inputs, training_outputs).set_noise(output_noise)
///                                                                     .set_kernel(exponential_kernel)
///                                                                     .fit_kernel()
///                                                                     .set_prior(linear_prior)
///                                                                     .fit_prior()
///                                                                     .train();
/// # }
/// ```
pub struct GaussianProcessBuilder<KernelType: Kernel, PriorType: Prior>
{
   /// value to which the process will regress in the absence of informations
   prior: PriorType,
   /// kernel used to fit the process on the data
   kernel: KernelType,
   /// amplitude of the noise of the data
   noise: f64,
   /// type of fit to be applied
   should_fit_kernel: bool,
   should_fit_prior: bool,
   /// data use for training
   training_inputs: DMatrix<f64>,
   training_outputs: DVector<f64>
}

impl<KernelType: Kernel, PriorType: Prior> GaussianProcessBuilder<KernelType, PriorType>
{
   /// builds a new gaussian process with default parameters
   ///
   /// the defaults are :
   /// - constant prior (0 unless fitted)
   /// - a gaussian kernel
   /// - a noise of 1e-7
   /// - does not fit parameters
   /// - does fit prior
   pub fn new<T: Input>(training_inputs: T, training_outputs: T::InVector) -> Self
   {
      let training_inputs = T::into_dmatrix(training_inputs);
      let training_outputs = T::into_dvector(training_outputs);
      // makes builder
      let prior = PriorType::default(training_inputs.ncols());
      let kernel = KernelType::default();
      let noise = 1e-7f64;
      let should_fit_kernel = false;
      let should_fit_prior = false;
      GaussianProcessBuilder { prior,
                               kernel,
                               noise,
                               should_fit_kernel,
                               should_fit_prior,
                               training_inputs,
                               training_outputs }
   }

   //----------------------------------------------------------------------------------------------
   // SETTERS

   /// Sets a new prior.
   /// See the documentation on priors for more informations.
   pub fn set_prior<PriorType2: Prior>(self,
                                       prior: PriorType2)
                                       -> GaussianProcessBuilder<KernelType, PriorType2>
   {
      GaussianProcessBuilder { prior,
                               kernel: self.kernel,
                               noise: self.noise,
                               should_fit_kernel: self.should_fit_kernel,
                               should_fit_prior: self.should_fit_prior,
                               training_inputs: self.training_inputs,
                               training_outputs: self.training_outputs }
   }

   /// Sets the noise parameter.
   /// It correspond to the standard deviation of the noise in the outputs of the training set.
   pub fn set_noise(self, noise: f64) -> Self
   {
      GaussianProcessBuilder { noise, ..self }
   }

   /// Changes the kernel of the gaussian process.
   /// See the documentations on Kernels for more informations.
   pub fn set_kernel<KernelType2: Kernel>(self,
                                          kernel: KernelType2)
                                          -> GaussianProcessBuilder<KernelType2, PriorType>
   {
      GaussianProcessBuilder { prior: self.prior,
                               kernel,
                               noise: self.noise,
                               should_fit_kernel: self.should_fit_kernel,
                               should_fit_prior: self.should_fit_prior,
                               training_inputs: self.training_inputs,
                               training_outputs: self.training_outputs }
   }

   /// Asks for the parameters of the kernel to be fit on the training data.
   /// The fitting will be done when the `train` method is called.
   pub fn fit_kernel(self) -> Self
   {
      GaussianProcessBuilder { should_fit_kernel: true, ..self }
   }

   /// Asks for the prior to be fit on the training data.
   /// The fitting will be done when the `train` method is called.
   pub fn fit_prior(self) -> Self
   {
      GaussianProcessBuilder { should_fit_prior: true, ..self }
   }

   //----------------------------------------------------------------------------------------------
   // TRAIN

   /// Trains the gaussian process.
   /// Fits the parameters if requested.
   pub fn train(self) -> GaussianProcess<KernelType, PriorType>
   {
      // builds a gp
      // TODO here we waste a training if we will fit anyway
      // TODO a new_fitted funtion might solve it
      // TODO or a raw construction that does not perform a fit
      let mut gp = GaussianProcess::<KernelType, PriorType>::new(self.prior,
                                                                 self.kernel,
                                                                 self.noise,
                                                                 self.training_inputs,
                                                                 self.training_outputs);
      // fit the model, if reqiested, on the training data
      gp.fit_parameters(self.should_fit_prior, self.should_fit_kernel);
      gp
   }
}
